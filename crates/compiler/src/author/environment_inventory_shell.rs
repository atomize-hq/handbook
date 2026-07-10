use super::{
    acquire_authoring_lock, baseline_authoring_eligibility, canonical_artifact_identity,
    environment_inventory::{
        map_environment_inventory_core_error, AuthorEnvironmentInventoryRefusal,
        AuthorEnvironmentInventoryRefusalKind, AuthorEnvironmentInventoryResult,
    },
    environment_inventory_core::{
        render_environment_inventory_markdown as render_environment_inventory_markdown_core,
        validate_synthesized_environment_inventory_markdown as validate_synthesized_environment_inventory_markdown_core,
        EnvironmentInventoryCoreError, EnvironmentInventoryCoreErrorKind,
        EnvironmentInventoryStructuredInput, EnvironmentInventoryValidationExpectations,
    },
    format_repo_mutation_error, format_repo_write_path_error, render_exit_status,
    summarize_process_output,
    template_library::resolve_template_library,
    template_library::TemplateLibraryRequest,
    template_library::TemplateLibraryResolveRequest,
    template_library::TemplateLibrarySelection,
    validate_canonical_write_target, validate_system_root_for_authoring, AuthoringLockError,
    BaselineAuthoringEligibility, SystemRootAuthoringError,
};
use crate::baseline_validation::{baseline_artifact_validation, BaselineArtifactVerdict};
use crate::canonical_artifacts::{CanonicalArtifactKind, CanonicalArtifacts};
use crate::layout::RepoLayoutRoot;
use crate::repo_file_access::write_repo_relative_bytes;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use time::macros::format_description;
use time::OffsetDateTime;

const AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN_ENV_VAR: &str =
    "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN";
const AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL_ENV_VAR: &str =
    "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL";
const AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC_ENV_VAR: &str =
    "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC";
const NOW_UTC_FORMAT: &[time::format_description::FormatItem<'static>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct EnvironmentInventorySynthesisInputs {
    charter_markdown: String,
    project_context_markdown: Option<String>,
}

pub(super) fn render_environment_inventory_markdown(
    input: &EnvironmentInventoryStructuredInput,
) -> Result<String, AuthorEnvironmentInventoryRefusal> {
    let now_utc = resolve_environment_inventory_now_utc().map_err(|summary| {
        map_environment_inventory_core_error(EnvironmentInventoryCoreError {
            kind: EnvironmentInventoryCoreErrorKind::DeterministicRenderFailed,
            summary,
        })
    })?;
    render_environment_inventory_markdown_core(input, &now_utc)
        .map_err(map_environment_inventory_core_error)
}

pub(super) fn preflight_author_environment_inventory(
    repo_root: &Path,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    let _ = prepare_environment_inventory_authoring_inputs(repo_root)?;
    Ok(())
}

pub(super) fn prepare_environment_inventory_authoring_inputs(
    repo_root: &Path,
) -> Result<EnvironmentInventorySynthesisInputs, AuthorEnvironmentInventoryRefusal> {
    let artifacts =
        CanonicalArtifacts::load(repo_root).map_err(|err| AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::InvalidSystemRoot,
            summary: format!("failed to inspect canonical `.handbook` root: {err}"),
            broken_subject: "canonical `.handbook` root".to_string(),
            next_safe_action: "repair the canonical `.handbook` root and rerun `handbook setup`"
                .to_string(),
        })?;

    validate_environment_inventory_authoring_preconditions(repo_root, &artifacts)?;

    let charter_markdown = required_charter_markdown(&artifacts)?;
    let project_context_markdown = optional_project_context_markdown(&artifacts)?;

    Ok(EnvironmentInventorySynthesisInputs {
        charter_markdown,
        project_context_markdown,
    })
}

pub(super) fn with_environment_inventory_authoring_lock<T, F>(
    repo_root: &Path,
    action: F,
) -> Result<T, AuthorEnvironmentInventoryRefusal>
where
    F: FnOnce() -> Result<T, AuthorEnvironmentInventoryRefusal>,
{
    let environment_inventory_layout = RepoLayoutRoot::new(repo_root)
        .authoring()
        .environment_inventory();
    let _lock =
        acquire_authoring_lock(repo_root, environment_inventory_layout.lock_path().as_str())
            .map_err(|err| map_authoring_lock_error(repo_root, err))?;
    action()
}

pub(super) fn write_canonical_environment_inventory_markdown(
    repo_root: &Path,
    markdown: &str,
) -> Result<AuthorEnvironmentInventoryResult, AuthorEnvironmentInventoryRefusal> {
    let environment_inventory_layout = RepoLayoutRoot::new(repo_root)
        .authoring()
        .environment_inventory();
    write_repo_relative_bytes(
        repo_root,
        environment_inventory_layout.canonical_target().as_str(),
        markdown.as_bytes(),
    )
    .map_err(|err| AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
        summary: format_repo_mutation_error(
            environment_inventory_layout.canonical_target_relative(),
            err,
        ),
        broken_subject: "canonical environment inventory write target".to_string(),
        next_safe_action:
            "repair the blocked canonical environment inventory path and retry `handbook author environment-inventory`"
                .to_string(),
    })?;

    Ok(AuthorEnvironmentInventoryResult {
        canonical_repo_relative_path: environment_inventory_layout.canonical_target_relative(),
        bytes_written: markdown.len(),
    })
}

pub(super) fn synthesize_environment_inventory_markdown(
    repo_root: &Path,
    inputs: &EnvironmentInventorySynthesisInputs,
) -> Result<String, AuthorEnvironmentInventoryRefusal> {
    let prompt = build_environment_inventory_synthesis_prompt(repo_root, inputs)?;
    let output_path = synthesize_output_path();
    let codex_bin = std::env::var(AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN_ENV_VAR)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "codex".to_string());
    let codex_model = std::env::var(AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL_ENV_VAR)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let mut command = Command::new(&codex_bin);
    command
        .current_dir(repo_root)
        .arg("exec")
        .arg("--skip-git-repo-check")
        .arg("--sandbox")
        .arg("read-only")
        .arg("--color")
        .arg("never");
    if let Some(model) = codex_model.as_deref() {
        command.arg("--model").arg(model);
    }
    command
        .arg("--output-last-message")
        .arg(&output_path)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|err| {
        synthesis_refusal(format!(
            "failed to start `codex exec` for environment inventory synthesis: {err}"
        ))
    })?;

    let prompt_write_result = {
        let stdin = child.stdin.as_mut().ok_or_else(|| {
            synthesis_refusal(
                "failed to open stdin for `codex exec` environment inventory synthesis",
            )
        })?;
        stdin.write_all(prompt.as_bytes())
    };
    if let Err(err) = prompt_write_result {
        let _ = child.kill();
        let _ = child.wait();
        let _ = std::fs::remove_file(&output_path);
        return Err(synthesis_refusal(format!(
            "failed to stream environment inventory synthesis prompt into `codex exec`: {err}"
        )));
    }

    let output = child.wait_with_output().map_err(|err| {
        synthesis_refusal(format!(
            "failed while waiting for `codex exec` environment inventory synthesis: {err}"
        ))
    })?;

    if !output.status.success() {
        let command_output = summarize_process_output(&output.stdout, &output.stderr);
        let _ = std::fs::remove_file(&output_path);
        return Err(synthesis_refusal(format!(
            "`codex exec` environment inventory synthesis exited with {}{}",
            render_exit_status(output.status.code()),
            command_output
        )));
    }

    let markdown = std::fs::read_to_string(&output_path).map_err(|err| {
        synthesis_refusal(format!(
            "failed to read synthesized environment inventory markdown from {}: {err}",
            output_path.display()
        ))
    })?;
    let _ = std::fs::remove_file(&output_path);

    let normalized = markdown.trim().to_string();
    let expectations = EnvironmentInventoryValidationExpectations::for_optional_project_context(
        inputs.project_context_markdown.is_some(),
    );
    validate_synthesized_environment_inventory_markdown_core(&normalized, expectations)
        .map_err(synthesis_refusal)?;
    Ok(normalized)
}

pub(super) fn synthesis_refusal(summary: impl Into<String>) -> AuthorEnvironmentInventoryRefusal {
    AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::SynthesisFailed,
        summary: summary.into(),
        broken_subject: "environment inventory synthesis runtime".to_string(),
        next_safe_action:
            "repair the environment inventory synthesis runtime or prompt inputs and retry `handbook author environment-inventory`"
                .to_string(),
    }
}

fn validate_environment_inventory_authoring_preconditions(
    repo_root: &Path,
    artifacts: &CanonicalArtifacts,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    let environment_inventory_layout = RepoLayoutRoot::new(repo_root)
        .authoring()
        .environment_inventory();
    match validate_system_root_for_authoring(artifacts) {
        Ok(()) => {}
        Err(SystemRootAuthoringError::Missing) => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::MissingSystemRoot,
                summary:
                    "canonical `.handbook` root is missing; environment inventory authoring requires setup first"
                        .to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action: "run `handbook setup`".to_string(),
            });
        }
        Err(SystemRootAuthoringError::NotDir) => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::InvalidSystemRoot,
                summary: "canonical `.handbook` root exists but is not a directory".to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action:
                    "repair the canonical `.handbook` root and rerun `handbook setup`".to_string(),
            });
        }
        Err(SystemRootAuthoringError::SymlinkNotAllowed) => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::InvalidSystemRoot,
                summary: "canonical `.handbook` root cannot be a symlink".to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action: "remove the `.handbook` symlink and rerun `handbook setup`"
                    .to_string(),
            });
        }
    }

    let environment_inventory =
        canonical_artifact_identity(artifacts, CanonicalArtifactKind::EnvironmentInventory);
    if environment_inventory.kind != CanonicalArtifactKind::EnvironmentInventory {
        return Err(AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth,
            summary: "unexpected canonical artifact identity for environment inventory authoring"
                .to_string(),
            broken_subject: "canonical environment inventory truth".to_string(),
            next_safe_action:
                "inspect canonical artifact metadata and retry `handbook author environment-inventory`"
                    .to_string(),
        });
    }

    match baseline_authoring_eligibility(artifacts, CanonicalArtifactKind::EnvironmentInventory) {
        BaselineAuthoringEligibility::Authorable => {}
        BaselineAuthoringEligibility::ExistingValidCanonicalTruth => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth,
                summary:
                    "canonical environment inventory truth already exists as valid non-starter truth; `handbook author environment-inventory` refuses to overwrite authored canonical truth"
                        .to_string(),
                broken_subject: environment_inventory_layout
                    .canonical_target_relative()
                    .to_string(),
                next_safe_action: format!(
                    "inspect `{}` instead of rerunning `handbook author environment-inventory`",
                    environment_inventory_layout.canonical_target_relative()
                ),
            });
        }
        BaselineAuthoringEligibility::RequiresSetupRefresh => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
                summary:
                    "canonical environment inventory truth is unreadable or path-invalid; repair it with `handbook setup refresh` before rerunning `handbook author environment-inventory`"
                        .to_string(),
                broken_subject: environment_inventory_layout
                    .canonical_target_relative()
                    .to_string(),
                next_safe_action: "run `handbook setup refresh`".to_string(),
            });
        }
    }

    validate_canonical_write_target(repo_root, environment_inventory_layout.canonical_target().as_str())
        .map_err(|err| AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
            summary: format_repo_write_path_error(
                environment_inventory_layout.canonical_target_relative(),
                err,
            ),
            broken_subject: "canonical environment inventory write target".to_string(),
            next_safe_action:
                "repair the blocked canonical environment inventory path and retry `handbook author environment-inventory`"
                    .to_string(),
        })?;

    Ok(())
}

fn required_charter_markdown(
    artifacts: &CanonicalArtifacts,
) -> Result<String, AuthorEnvironmentInventoryRefusal> {
    let validation = baseline_artifact_validation(artifacts, CanonicalArtifactKind::Charter)
        .expect("charter must be part of baseline validation");

    match validation.verdict {
        BaselineArtifactVerdict::Missing => Err(AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter,
            summary:
                "canonical charter truth is missing; environment inventory authoring requires a completed charter first"
                    .to_string(),
            broken_subject: ".handbook/charter/CHARTER.md".to_string(),
            next_safe_action: "run `handbook author charter`".to_string(),
        }),
        BaselineArtifactVerdict::Empty => Err(AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter,
            summary:
                "canonical charter truth is empty; environment inventory authoring requires a completed charter first"
                    .to_string(),
            broken_subject: ".handbook/charter/CHARTER.md".to_string(),
            next_safe_action: "run `handbook author charter`".to_string(),
        }),
        BaselineArtifactVerdict::StarterOwned => Err(AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter,
            summary:
                "canonical charter truth still contains the shipped starter template; environment inventory authoring requires a completed charter first"
                    .to_string(),
            broken_subject: ".handbook/charter/CHARTER.md".to_string(),
            next_safe_action: "run `handbook author charter`".to_string(),
        }),
        BaselineArtifactVerdict::IngestInvalid => Err(invalid_upstream_canonical_truth_refusal(
            ".handbook/charter/CHARTER.md",
            "canonical charter truth is unreadable or non-canonical; environment inventory authoring requires valid charter truth".to_string(),
            "run `handbook setup refresh`".to_string(),
        )),
        BaselineArtifactVerdict::SemanticallyInvalid { summary } => {
            Err(invalid_upstream_canonical_truth_refusal(
                ".handbook/charter/CHARTER.md",
                format!("canonical charter truth is invalid: {summary}"),
                "run `handbook author charter`".to_string(),
            ))
        }
        BaselineArtifactVerdict::ValidCanonicalTruth { markdown } => Ok(markdown),
    }
}

fn optional_project_context_markdown(
    artifacts: &CanonicalArtifacts,
) -> Result<Option<String>, AuthorEnvironmentInventoryRefusal> {
    let validation = baseline_artifact_validation(artifacts, CanonicalArtifactKind::ProjectContext)
        .expect("project context must be part of baseline validation");

    match validation.verdict {
        BaselineArtifactVerdict::Missing
        | BaselineArtifactVerdict::Empty
        | BaselineArtifactVerdict::StarterOwned => Ok(None),
        BaselineArtifactVerdict::IngestInvalid => Err(invalid_upstream_canonical_truth_refusal(
            ".handbook/project_context/PROJECT_CONTEXT.md",
            "canonical project context truth is unreadable or non-canonical; repair it or remove it before environment inventory authoring"
                .to_string(),
            "run `handbook setup refresh`".to_string(),
        )),
        BaselineArtifactVerdict::SemanticallyInvalid { summary } => {
            Err(invalid_upstream_canonical_truth_refusal(
                ".handbook/project_context/PROJECT_CONTEXT.md",
                format!("canonical project context truth is invalid: {summary}"),
                "run `handbook author project-context`".to_string(),
            ))
        }
        BaselineArtifactVerdict::ValidCanonicalTruth { markdown } => {
            let trimmed = markdown.trim();
            if trimmed.is_empty() {
                Ok(None)
            } else {
                Ok(Some(trimmed.to_string()))
            }
        }
    }
}

fn build_environment_inventory_synthesis_prompt(
    repo_root: &Path,
    inputs: &EnvironmentInventorySynthesisInputs,
) -> Result<String, AuthorEnvironmentInventoryRefusal> {
    let selection = match resolve_template_library(
        repo_root,
        &TemplateLibraryResolveRequest::new(TemplateLibraryRequest::EnvironmentInventoryAuthoring),
    )
    .map_err(|err| {
        synthesis_refusal(format!(
            "failed to resolve environment inventory authoring assets: {err}"
        ))
    })? {
        TemplateLibrarySelection::EnvironmentInventory(selection) => selection,
        TemplateLibrarySelection::Charter(_) => {
            unreachable!(
                "environment inventory authoring must resolve environment-inventory template-library assets"
            )
        }
    };
    let project_context_ref = if inputs.project_context_markdown.is_some() {
        ".handbook/project_context/PROJECT_CONTEXT.md"
    } else {
        "None"
    };

    let mut prompt = String::new();
    prompt.push_str(
        "# Environment Inventory Synthesis Directive

",
    );
    prompt.push_str(selection.synthesize_directive().contents());
    prompt.push_str(
        "

# Canonical Write Contract

",
    );
    prompt.push_str("- Write only the canonical `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` content.
");
    prompt.push_str(
        "- Do not describe any repo-root `ENVIRONMENT_INVENTORY.md` file as canonical.
",
    );
    prompt.push_str(
        "- Do not mention artifact copies as canonical or as the store of record.
",
    );
    prompt.push_str("- `PROJECT_CONTEXT` is optional context for this authoring flow. If it is absent, continue from the charter alone.
");
    prompt.push_str(
        "
# ENVIRONMENT_INVENTORY.md Template

```md
",
    );
    prompt.push_str(selection.template().contents().trim());
    prompt.push_str(
        "
```

",
    );
    prompt.push_str(
        "# Exact references that must be preserved verbatim

",
    );
    prompt.push_str(
        "- Canonical file reference: `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`
",
    );
    prompt.push_str(&format!(
        "- Project context reference line: `{project_context_ref}`
"
    ));
    prompt.push_str(
        "
# CHARTER.md

```md
",
    );
    prompt.push_str(inputs.charter_markdown.trim());
    prompt.push_str(
        "
```
",
    );

    if let Some(project_context_markdown) = &inputs.project_context_markdown {
        prompt.push_str(
            "
# PROJECT_CONTEXT.md (optional context)

```md
",
        );
        prompt.push_str(project_context_markdown.trim());
        prompt.push_str(
            "
```
",
        );
    } else {
        prompt.push_str(
            "
# PROJECT_CONTEXT.md (optional context)

Not present for this authoring run.
",
        );
    }

    prompt.push_str(
        "
Return only the final `ENVIRONMENT_INVENTORY.md` markdown with all template placeholders resolved.
",
    );
    Ok(prompt)
}

fn invalid_upstream_canonical_truth_refusal(
    broken_subject: &str,
    summary: String,
    next_safe_action: String,
) -> AuthorEnvironmentInventoryRefusal {
    AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::InvalidUpstreamCanonicalTruth,
        summary,
        broken_subject: broken_subject.to_string(),
        next_safe_action,
    }
}

fn resolve_environment_inventory_now_utc() -> Result<String, String> {
    if let Ok(value) = std::env::var(AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC_ENV_VAR) {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    OffsetDateTime::now_utc()
        .format(NOW_UTC_FORMAT)
        .map_err(|error| {
            format!("failed to derive environment-inventory render timestamp: {error}")
        })
}

fn map_authoring_lock_error(
    repo_root: &Path,
    err: AuthoringLockError,
) -> AuthorEnvironmentInventoryRefusal {
    let environment_inventory_layout = RepoLayoutRoot::new(repo_root)
        .authoring()
        .environment_inventory();
    match err {
        AuthoringLockError::WritePath(path_err) => AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
            summary: format_repo_write_path_error(
                environment_inventory_layout.lock_relative_path(),
                path_err,
            ),
            broken_subject: "environment inventory authoring lock".to_string(),
            next_safe_action:
                "repair the blocked environment inventory authoring lock path and retry `handbook author environment-inventory`"
                    .to_string(),
        },
        AuthoringLockError::Io { lock_path, source } => AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
            summary: format!(
                "failed to acquire exclusive environment inventory authoring lock at {}: {source}",
                lock_path.display()
            ),
            broken_subject: "environment inventory authoring lock".to_string(),
            next_safe_action:
                "wait for any in-progress `handbook author environment-inventory` run to finish or repair the lock path, then retry `handbook author environment-inventory`"
                    .to_string(),
        },
    }
}

fn synthesize_output_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "handbook-author-environment-inventory-{}-{timestamp}.md",
        std::process::id()
    ))
}
