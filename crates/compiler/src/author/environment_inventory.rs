use super::{
    environment_inventory_core::{
        parse_environment_inventory_structured_input_yaml as parse_environment_inventory_structured_input_yaml_core,
        validate_environment_inventory_markdown as validate_environment_inventory_markdown_core,
        validate_environment_inventory_structured_input as validate_environment_inventory_structured_input_core,
        EnvironmentInventoryCoreError, EnvironmentInventoryCoreErrorKind,
    },
    environment_inventory_shell::{
        preflight_author_environment_inventory as preflight_author_environment_inventory_shell,
        prepare_environment_inventory_authoring_inputs,
        render_environment_inventory_markdown as render_environment_inventory_markdown_shell,
        synthesis_refusal as synthesis_refusal_shell, synthesize_environment_inventory_markdown,
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
    preflight_author_environment_inventory_shell(repo_root.as_ref())
}

pub fn preflight_author_environment_inventory_from_input(
    repo_root: impl AsRef<Path>,
    input: &EnvironmentInventoryStructuredInput,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    validate_environment_inventory_structured_input(input)?;
    preflight_author_environment_inventory(repo_root)
}

pub fn author_environment_inventory(
    repo_root: impl AsRef<Path>,
) -> Result<AuthorEnvironmentInventoryResult, AuthorEnvironmentInventoryRefusal> {
    let repo_root = repo_root.as_ref();
    preflight_author_environment_inventory(repo_root)?;
    with_environment_inventory_authoring_lock(repo_root, || {
        let inputs = prepare_environment_inventory_authoring_inputs(repo_root)?;
        let markdown = synthesize_environment_inventory_markdown(repo_root, &inputs)?;
        write_canonical_environment_inventory_markdown(repo_root, &markdown)
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
    validate_environment_inventory_markdown_core(markdown).map_err(synthesis_refusal_shell)
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

#[cfg(test)]
mod tests {
    use super::{
        author_environment_inventory, preflight_author_environment_inventory,
        AuthorEnvironmentInventoryRefusalKind, CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH,
    };
    use crate::canonical_artifacts::setup_starter_template;
    use crate::CanonicalArtifactKind;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;
    use tempfile::tempdir;

    fn valid_charter_markdown() -> &'static str {
        "# Engineering Charter — Handbook

## What this is
Body.

## How to use this charter
Use it.

## Rubric: 1–5 rigor levels
Levels.

## Project baseline posture
Baseline.

## Domains / areas (optional overrides)
None.

## Posture at a glance (quick scan)
Snapshot.

## Dimensions (details + guardrails)
Details.

## Cross-cutting red lines (global non-negotiables)
- Keep trust boundaries intact.

## Exceptions / overrides process
- **Approvers:** project_owner
- **Record location:** docs/exceptions.md
- **Minimum required fields:**
  - what
  - why
  - scope
  - risk
  - owner
  - expiry_or_revisit_date

## Debt tracking expectations
Tracked in issues.

## Decision Records (ADRs): how to use this charter
Use ADRs.

## Review & updates
Review monthly.
"
    }

    fn valid_project_context_markdown() -> &'static str {
        "# Project Context — Handbook

> **File:** `PROJECT_CONTEXT.md`
> **Created (UTC):** 2026-04-21T00:00:00Z
> **Owner:** project-owner
> **Team:** handbook-team
> **Repo / Project:** /tmp/handbook
> **Charter Ref:** .handbook/charter/CHARTER.md

## What this is
Project reality.

## How to use this
Use this document to ground planning in reality.

## 0) Project Summary (factual, 3–6 bullets)
- Summary.

## 1) Operational Reality (the most important section)
- Runs on macOS and Linux.

## 2) Project Classification Implications (planning guardrails)
- Guardrails.

## 3) System Boundaries (what we own vs integrate with)
### What we own
- Canonical `.handbook/` truth.
### What we do NOT own (but may depend on)
- External delivery systems.

## 4) Integrations & Contracts (top 1–5)
- Integrations.

## 5) Environments & Delivery
- Delivery.

## 6) Data Reality
- Data.

## 7) Repo / Codebase Reality (brownfield-friendly, but safe for greenfield)
- Codebase.

## 8) Constraints
- Constraints.

## 9) Known Unknowns (explicitly tracked)
- Unknowns.

## 10) Update Triggers
- Update when reality changes.
"
    }

    #[test]
    fn preflight_requires_completed_charter() {
        let repo = tempdir().expect("tempdir");
        scaffold_environment_inventory_target(repo.path());

        let refusal = preflight_author_environment_inventory(repo.path()).expect_err("refusal");
        assert_eq!(
            refusal.kind,
            AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter
        );
        assert!(refusal.summary.contains("completed charter"));
    }

    #[test]
    fn author_writes_canonical_environment_inventory_and_reads_optional_project_context() {
        let repo = tempdir().expect("tempdir");
        scaffold_environment_inventory_target(repo.path());
        write_charter(repo.path(), valid_charter_markdown());
        write_project_context(repo.path(), valid_project_context_markdown());

        let prompt_log = repo.path().join("prompt.log");
        let fake_codex = write_fake_codex(repo.path());
        let canonical_markdown = r#"# Environment Inventory - Example

> **Canonical File:** `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`
> **Project Context Ref:** `.handbook/project_context/PROJECT_CONTEXT.md`

## What this is
The canonical store of record for this project's environment and runtime requirements.

## How to use
- Update this file whenever runtime assumptions change.

## 1) Environment Variables (Inventory)
- None yet.

## 2) External Services / Infrastructure Dependencies
- None yet.

## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)
- None yet.

## 4) Local Development Requirements
- None yet.

## 5) CI Requirements
- None yet.

## 6) Production / Deployment Requirements (even if not live yet)
- None yet.

## 7) Dependency & Tooling Inventory (project-specific)
- None yet.

## 8) Update Contract (non-negotiable)
- Update `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` in the same change.

## 9) Known Unknowns
- None yet.
"#;

        std::env::set_var(
            "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN",
            &fake_codex,
        );
        std::env::set_var(
            "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_TEST_OUTPUT",
            canonical_markdown,
        );
        std::env::set_var(
            "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_TEST_PROMPT_LOG",
            &prompt_log,
        );

        let result = author_environment_inventory(repo.path()).expect("author success");
        assert_eq!(
            result.canonical_repo_relative_path,
            CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH
        );
        assert_eq!(
            fs::read_to_string(repo.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH))
                .expect("inventory")
                .trim(),
            canonical_markdown.trim()
        );

        let prompt = fs::read_to_string(&prompt_log).expect("prompt log");
        assert!(prompt.contains(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"));
        assert!(prompt.contains("Runs on macOS and Linux."));

        std::env::remove_var("HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN");
        std::env::remove_var("HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_TEST_OUTPUT");
        std::env::remove_var("HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_TEST_PROMPT_LOG");
    }

    #[test]
    fn author_refuses_to_overwrite_existing_non_starter_truth() {
        let repo = tempdir().expect("tempdir");
        scaffold_environment_inventory_target(repo.path());
        write_charter(repo.path(), valid_charter_markdown());
        fs::write(
            repo.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH),
            valid_environment_inventory_markdown(),
        )
        .expect("write inventory");

        let refusal = author_environment_inventory(repo.path()).expect_err("refusal");
        assert_eq!(
            refusal.kind,
            AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth
        );
    }

    fn scaffold_environment_inventory_target(repo_root: &Path) {
        fs::create_dir_all(repo_root.join(".handbook/environment_inventory")).expect("mkdir");
        fs::create_dir_all(repo_root.join(".handbook/charter")).expect("mkdir");
        fs::write(
            repo_root.join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH),
            setup_starter_template(CanonicalArtifactKind::EnvironmentInventory),
        )
        .expect("starter inventory");
    }

    fn write_charter(repo_root: &Path, content: &str) {
        fs::write(repo_root.join(".handbook/charter/CHARTER.md"), content).expect("write charter");
    }

    fn write_project_context(repo_root: &Path, content: &str) {
        fs::create_dir_all(repo_root.join(".handbook/project_context")).expect("mkdir");
        fs::write(
            repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
            content,
        )
        .expect("write project context");
    }

    fn valid_environment_inventory_markdown() -> &'static str {
        r#"# Environment Inventory - Example

> **Canonical File:** `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`
> **Project Context Ref:** None

## What this is
The canonical store of record for this project's environment and runtime requirements.

## How to use
- Update this file whenever runtime assumptions change.

## 1) Environment Variables (Inventory)
- None yet.

## 2) External Services / Infrastructure Dependencies
- None yet.

## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)
- None yet.

## 4) Local Development Requirements
- None yet.

## 5) CI Requirements
- None yet.

## 6) Production / Deployment Requirements (even if not live yet)
- None yet.

## 7) Dependency & Tooling Inventory (project-specific)
- None yet.

## 8) Update Contract (non-negotiable)
- Update `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` in the same change.

## 9) Known Unknowns
- None yet.
"#
    }

    fn write_fake_codex(repo_root: &Path) -> String {
        let script_path = repo_root.join("fake-codex.sh");
        let script = r#"#!/bin/sh
output_path=""
while [ "$#" -gt 0 ]; do
  if [ "$1" = "--output-last-message" ]; then
    shift
    output_path="$1"
  fi
  shift
done
cat > "${HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_TEST_PROMPT_LOG}"
printf '%s' "${HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_TEST_OUTPUT}" > "$output_path"
"#;
        fs::write(&script_path, script).expect("write fake codex");
        let mut perms = fs::metadata(&script_path).expect("metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).expect("chmod");
        script_path.display().to_string()
    }
}
