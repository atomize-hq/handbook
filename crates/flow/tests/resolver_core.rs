use handbook_flow::{
    resolve, BudgetDisposition, BudgetPolicy, PacketSectionMode, PacketSelectionStatus,
    PacketVariant, ResolveRequest, ResolverNextSafeAction, ResolverRefusalCategory,
    ResolverSubjectRef,
};
use handbook_engine::CanonicalArtifactKind;

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

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
- Operations.

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

fn valid_environment_inventory_markdown() -> &'static str {
    "# Environment Inventory

> **Canonical File:** `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`
> **Project Context Ref:** `.handbook/project_context/PROJECT_CONTEXT.md`

## What this is
Canonical environment and runtime inventory.

## How to use
- Update this file when runtime assumptions change.

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
"
}

fn oversized_valid_project_context_markdown() -> String {
    format!("{}\n{}", valid_project_context_markdown(), "x".repeat(256))
}

#[test]
fn flow_resolver_blocks_missing_system_root_with_typed_refusal() {
    let dir = tempfile::tempdir().expect("tempdir");

    let result = resolve(dir.path(), ResolveRequest::default()).expect("resolve");

    assert_eq!(result.selection.status, PacketSelectionStatus::Blocked);
    assert!(result.packet_result.sections.is_empty());
    assert!(result
        .packet_result
        .notes
        .iter()
        .any(|note| note.text == "packet body omitted because request is not ready"));
    assert_eq!(
        result.refusal.as_ref().map(|refusal| refusal.category),
        Some(ResolverRefusalCategory::SystemRootMissing)
    );
}

#[test]
fn flow_resolver_prioritizes_system_root_missing_over_live_execution_refusal() {
    let dir = tempfile::tempdir().expect("tempdir");

    let result = resolve(
        dir.path(),
        ResolveRequest {
            packet_id: "execution.live.packet",
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");

    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, ResolverRefusalCategory::SystemRootMissing);
}

#[test]
fn flow_resolver_builds_ready_planning_packet_body() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature spec body",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");

    assert!(result.packet_result.is_ready());
    assert_eq!(result.packet_result.variant, PacketVariant::Planning);
    assert!(result.packet_result.fixture_context.is_none());
    assert_eq!(result.packet_result.included_sources.len(), 3);
    assert_eq!(result.packet_result.sections.len(), 3);
    assert_eq!(result.packet_result.sections[0].title, "CHARTER");
    assert_eq!(result.packet_result.sections[1].title, "PROJECT_CONTEXT");
    assert_eq!(result.packet_result.sections[2].title, "FEATURE_SPEC");
    assert_eq!(result.packet_result.sections[0].mode, PacketSectionMode::Verbatim);
    assert_eq!(result.packet_result.sections[0].contents, valid_charter_markdown());
    assert_eq!(
        result.packet_result.decision_summary.ready_next_safe_action,
        "run `handbook inspect --packet planning.packet` for proof"
    );
}

#[test]
fn flow_resolver_summarizes_optional_sources_when_budget_demands_it() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        oversized_valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature spec body",
    );

    let result = resolve(
        root,
        ResolveRequest {
            budget_policy: BudgetPolicy {
                max_total_bytes: None,
                max_per_artifact_bytes: Some(1000),
            },
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");

    assert_eq!(result.budget_outcome.disposition, BudgetDisposition::Summarize);
    let section = result
        .packet_result
        .sections
        .iter()
        .find(|section| section.title == "PROJECT_CONTEXT")
        .expect("project context section");
    assert_eq!(section.mode, PacketSectionMode::Summary);
    assert!(section.contents.contains("budget summary: full contents omitted"));
    assert!(result.packet_result.notes.iter().any(|note| {
        note.text
            == "optional source summarized due to budget: .handbook/project_context/PROJECT_CONTEXT.md"
    }));
}

#[cfg(unix)]
#[test]
fn flow_resolver_refuses_symlinked_canonical_artifact_as_non_canonical_input() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    std::fs::create_dir_all(root.join(".handbook/charter")).expect("mkdirs");
    std::fs::create_dir_all(root.join(".handbook/feature_spec")).expect("mkdirs");

    let real = root.join("real_charter.md");
    write_file(&real, b"charter");
    symlink(&real, root.join(".handbook/charter/CHARTER.md")).expect("symlink charter");
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");

    let refusal = result.refusal.expect("refusal");
    assert_eq!(
        refusal.category,
        ResolverRefusalCategory::NonCanonicalInputAttempt
    );
    assert_eq!(
        refusal.broken_subject,
        ResolverSubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/charter/CHARTER.md",
        }
    );
    assert_eq!(refusal.next_safe_action, ResolverNextSafeAction::RunSetupRefresh);
}

#[test]
fn flow_resolver_blocks_optional_artifact_read_error_without_refusal() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    std::fs::create_dir_all(root.join(".handbook/project_context/PROJECT_CONTEXT.md"))
        .expect("project_context dir");

    let result = resolve(root, ResolveRequest::default()).expect("resolve");

    assert_eq!(result.selection.status, PacketSelectionStatus::Blocked);
    assert!(result.refusal.is_none());
    assert!(result.packet_result.sections.is_empty());
    assert!(
        !result.packet_result.notes.iter().any(|note| {
            note.text == "optional source omitted: .handbook/project_context/PROJECT_CONTEXT.md"
        }),
        "read errors must not be mislabeled as benign omissions: {:?}",
        result.packet_result.notes
    );
    assert!(result
        .packet_result
        .notes
        .iter()
        .any(|note| note.text == "packet body omitted because request is not ready"));
    assert!(result.blockers.iter().any(|blocker| {
        blocker.summary == "failed to read canonical artifact"
            && blocker.subject
                == ResolverSubjectRef::CanonicalArtifact {
                    kind: CanonicalArtifactKind::ProjectContext,
                    canonical_repo_relative_path:
                        ".handbook/project_context/PROJECT_CONTEXT.md",
                }
            && blocker.next_safe_action == ResolverNextSafeAction::RunSetupRefresh
    }));
}

#[test]
fn flow_resolver_refuses_required_artifact_malformed_path_read_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    std::fs::create_dir_all(root.join(".handbook/charter/CHARTER.md")).expect("charter dir");
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");

    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, ResolverRefusalCategory::ArtifactReadError);
    assert_eq!(
        refusal.broken_subject,
        ResolverSubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/charter/CHARTER.md",
        }
    );
    assert_eq!(refusal.next_safe_action, ResolverNextSafeAction::RunSetupRefresh);
}

#[test]
fn flow_resolver_refuses_when_budget_is_exhausted() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature spec that is longer than one byte",
    );

    let result = resolve(
        root,
        ResolveRequest {
            budget_policy: BudgetPolicy {
                max_total_bytes: None,
                max_per_artifact_bytes: Some(1),
            },
            packet_id: "planning.packet",
        },
    )
    .expect("resolve");

    assert_eq!(result.budget_outcome.disposition, BudgetDisposition::Refuse);
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, ResolverRefusalCategory::BudgetRefused);
    assert_eq!(
        refusal.broken_subject,
        ResolverSubjectRef::Policy {
            policy_id: "budget",
        }
    );
    assert!(matches!(
        refusal.next_safe_action,
        ResolverNextSafeAction::ReduceCanonicalArtifactSize { .. }
    ));
}

#[test]
fn flow_resolver_refuses_live_execution_packets_without_fixture_backing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(
        root,
        ResolveRequest {
            packet_id: "execution.live.packet",
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");

    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, ResolverRefusalCategory::UnsupportedRequest);
    assert!(
        refusal.summary.contains("fixture-backed"),
        "expected boundary statement mentioning fixture-backed demos: {:?}",
        refusal.summary
    );
    assert!(
        refusal.summary.contains("planning"),
        "expected boundary statement mentioning planning packets: {:?}",
        refusal.summary
    );
}

#[test]
fn flow_resolver_excludes_optional_sources_when_total_budget_demands_it() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        valid_environment_inventory_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature spec body",
    );

    let result = resolve(
        root,
        ResolveRequest {
            budget_policy: BudgetPolicy {
                max_total_bytes: Some(1),
                max_per_artifact_bytes: None,
            },
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");

    assert_eq!(result.budget_outcome.disposition, BudgetDisposition::Exclude);
    assert!(result
        .packet_result
        .sections
        .iter()
        .all(|section| section.title != "PROJECT_CONTEXT"));
    assert!(result.packet_result.notes.iter().any(|note| {
        note.text == "optional source excluded due to budget: .handbook/project_context/PROJECT_CONTEXT.md"
    }));
}

#[test]
fn flow_resolver_builds_fixture_context_for_execution_demo_packets() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().join("tests/fixtures/execution_demo/basic");

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        valid_environment_inventory_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"demo feature body",
    );

    let result = resolve(
        &root,
        ResolveRequest {
            packet_id: "execution.demo.packet",
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");

    assert!(result.packet_result.is_ready());
    assert_eq!(result.packet_result.variant, PacketVariant::ExecutionDemo);
    let fixture_context = result
        .packet_result
        .fixture_context
        .as_ref()
        .expect("fixture context");
    assert_eq!(fixture_context.fixture_set_id, "basic");
    assert_eq!(
        fixture_context.fixture_basis_root,
        "tests/fixtures/execution_demo/basic/.handbook/"
    );
    assert_eq!(fixture_context.fixture_lineage.len(), 4);
    assert_eq!(
        result.packet_result.decision_summary.ready_next_safe_action,
        "run `handbook inspect --packet execution.demo.packet --fixture-set basic` for proof"
    );
}
