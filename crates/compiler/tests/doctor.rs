use handbook_compiler::{
    doctor, BlockerCategory, CanonicalArtifactKind, DoctorArtifactStatus, DoctorBaselineStatus,
    NextSafeAction, SubjectRef, C04_RESULT_VERSION,
};
use handbook_engine::{
    setup_starter_template_bytes, C03_SCHEMA_VERSION, MANIFEST_GENERATION_VERSION,
};

fn assert_json_object_keys(
    value: &serde_json::Value,
    expected_keys: &[&str],
) -> serde_json::Map<String, serde_json::Value> {
    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("expected JSON object, got {value:?}"));
    let mut actual_keys = object.keys().map(String::as_str).collect::<Vec<_>>();
    actual_keys.sort_unstable();
    let mut expected_keys = expected_keys.to_vec();
    expected_keys.sort_unstable();
    assert_eq!(actual_keys, expected_keys);
    object.clone()
}

fn assert_doctor_report_json_contract(
    report: &handbook_compiler::DoctorReport,
) -> serde_json::Value {
    let value = serde_json::to_value(report).expect("serialize doctor report");
    assert_json_object_keys(
        &value,
        &[
            "c04_result_version",
            "c03_schema_version",
            "c03_manifest_generation_version",
            "baseline_state",
            "blockers",
            "status",
            "system_root_status",
            "checklist",
            "next_safe_action",
        ],
    );
    value
}

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

fn expected_artifact_label(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => "CHARTER",
        CanonicalArtifactKind::ProjectContext => "PROJECT_CONTEXT",
        CanonicalArtifactKind::EnvironmentInventory => "ENVIRONMENT_INVENTORY",
        CanonicalArtifactKind::FeatureSpec => "FEATURE_SPEC",
    }
}

fn expected_author_command(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => "run `handbook author charter --from-inputs <path|->`",
        CanonicalArtifactKind::ProjectContext => {
            "run `handbook author project-context --from-inputs <path|->`"
        }
        CanonicalArtifactKind::EnvironmentInventory => {
            "run `handbook author environment-inventory --from-inputs <path|->`"
        }
        CanonicalArtifactKind::FeatureSpec => {
            "fill canonical artifact at .handbook/feature_spec/FEATURE_SPEC.md"
        }
    }
}

fn assert_checklist_contract_fields(
    item: &handbook_compiler::DoctorChecklistItem,
    kind: CanonicalArtifactKind,
    canonical_repo_relative_path: &str,
) {
    assert_eq!(item.artifact_label, expected_artifact_label(kind));
    assert_eq!(item.author_command, expected_author_command(kind));
    match &item.subject {
        SubjectRef::CanonicalArtifact {
            kind: actual_kind,
            canonical_repo_relative_path: actual_path,
        } => {
            assert_eq!(*actual_kind, kind);
            assert_eq!(*actual_path, canonical_repo_relative_path);
        }
        other => panic!("expected canonical artifact subject, got {other:?}"),
    }
}

fn assert_empty_baseline_invalid(
    empty_path: &str,
    empty_kind: CanonicalArtifactKind,
    expected_action: NextSafeAction,
) {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        valid_environment_inventory_markdown().as_bytes(),
    );
    write_file(&repo_root.join(empty_path), b"");

    let report = doctor(repo_root).expect("doctor");
    assert_eq!(report.c04_result_version, C04_RESULT_VERSION);
    assert_eq!(report.c03_schema_version, C03_SCHEMA_VERSION);
    assert_eq!(
        report.c03_manifest_generation_version,
        MANIFEST_GENERATION_VERSION
    );
    assert_eq!(report.baseline_state, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(report.status, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(report.next_safe_action, Some(expected_action.clone()));

    let item = report
        .checklist
        .iter()
        .find(|item| item.kind == empty_kind)
        .expect("empty artifact");
    assert_checklist_contract_fields(item, empty_kind, empty_path);
    assert_eq!(item.status, DoctorArtifactStatus::Empty);
    assert_eq!(item.next_safe_action, Some(expected_action));
    assert_eq!(report.blockers.len(), 1);
    assert_eq!(
        report.blockers[0].category,
        BlockerCategory::RequiredArtifactEmpty
    );
    match &report.blockers[0].subject {
        SubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path,
        } => {
            assert_eq!(*kind, empty_kind);
            assert_eq!(*canonical_repo_relative_path, empty_path);
        }
        other => panic!("expected canonical artifact subject, got {other:?}"),
    }
}

#[test]
fn doctor_marks_only_project_context_invalid_for_matching_directory_ingest_issue() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    std::fs::create_dir_all(repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"))
        .expect("project_context dir");
    write_file(
        &repo_root.join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        valid_environment_inventory_markdown().as_bytes(),
    );

    let report = doctor(repo_root).expect("doctor");
    assert_eq!(report.baseline_state, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(report.status, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(
        report.next_safe_action,
        Some(NextSafeAction::RunSetupRefresh)
    );
    assert_eq!(report.checklist.len(), 3);

    assert_eq!(report.checklist[0].kind, CanonicalArtifactKind::Charter);
    assert_checklist_contract_fields(
        &report.checklist[0],
        CanonicalArtifactKind::Charter,
        ".handbook/charter/CHARTER.md",
    );
    assert_eq!(
        report.checklist[0].status,
        DoctorArtifactStatus::ValidCanonicalTruth
    );
    assert_eq!(report.checklist[0].next_safe_action, None);

    assert_eq!(
        report.checklist[1].kind,
        CanonicalArtifactKind::ProjectContext
    );
    assert_checklist_contract_fields(
        &report.checklist[1],
        CanonicalArtifactKind::ProjectContext,
        ".handbook/project_context/PROJECT_CONTEXT.md",
    );
    assert_eq!(report.checklist[1].status, DoctorArtifactStatus::Invalid);
    assert_eq!(
        report.checklist[1].next_safe_action,
        Some(NextSafeAction::RunSetupRefresh)
    );

    assert_eq!(
        report.checklist[2].kind,
        CanonicalArtifactKind::EnvironmentInventory
    );
    assert_checklist_contract_fields(
        &report.checklist[2],
        CanonicalArtifactKind::EnvironmentInventory,
        ".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md",
    );
    assert_eq!(
        report.checklist[2].status,
        DoctorArtifactStatus::ValidCanonicalTruth
    );
    assert_eq!(report.checklist[2].next_safe_action, None);
    assert_eq!(report.blockers.len(), 1);
    assert_eq!(
        report.blockers[0].category,
        BlockerCategory::ArtifactReadError
    );
    assert_eq!(
        report.blockers[0].subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::ProjectContext,
            canonical_repo_relative_path: ".handbook/project_context/PROJECT_CONTEXT.md",
        }
    );
    assert_eq!(
        report.blockers[0].summary,
        "failed to read canonical artifact"
    );
}

#[cfg(unix)]
#[test]
fn doctor_marks_only_environment_inventory_invalid_for_matching_symlink_ingest_issue() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        valid_project_context_markdown().as_bytes(),
    );
    std::fs::create_dir_all(repo_root.join(".handbook/environment_inventory")).expect("mkdirs");
    let redirected = repo_root.join("redirected_environment_inventory.md");
    write_file(
        &redirected,
        valid_environment_inventory_markdown().as_bytes(),
    );
    symlink(
        &redirected,
        repo_root.join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
    )
    .expect("symlink environment inventory");

    let report = doctor(repo_root).expect("doctor");
    assert_eq!(report.baseline_state, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(report.status, DoctorBaselineStatus::InvalidBaseline);
    assert_eq!(
        report.next_safe_action,
        Some(NextSafeAction::RunSetupRefresh)
    );

    assert_eq!(
        report
            .checklist
            .iter()
            .find(|item| item.kind == CanonicalArtifactKind::Charter)
            .expect("charter")
            .status,
        DoctorArtifactStatus::ValidCanonicalTruth
    );
    let environment_inventory = report
        .checklist
        .iter()
        .find(|item| item.kind == CanonicalArtifactKind::EnvironmentInventory)
        .expect("environment inventory");
    assert_checklist_contract_fields(
        environment_inventory,
        CanonicalArtifactKind::EnvironmentInventory,
        ".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md",
    );
    assert_eq!(environment_inventory.status, DoctorArtifactStatus::Invalid);
    assert_eq!(
        environment_inventory.next_safe_action,
        Some(NextSafeAction::RunSetupRefresh)
    );
    assert_eq!(report.blockers.len(), 1);
    assert_eq!(
        report.blockers[0].category,
        BlockerCategory::ArtifactReadError
    );
}

#[test]
fn doctor_treats_empty_charter_as_invalid_baseline() {
    assert_empty_baseline_invalid(
        ".handbook/charter/CHARTER.md",
        CanonicalArtifactKind::Charter,
        NextSafeAction::RunAuthorCharter,
    );
}

#[test]
fn doctor_treats_empty_project_context_as_invalid_baseline() {
    assert_empty_baseline_invalid(
        ".handbook/project_context/PROJECT_CONTEXT.md",
        CanonicalArtifactKind::ProjectContext,
        NextSafeAction::RunAuthorProjectContext,
    );
}

#[test]
fn doctor_treats_empty_environment_inventory_as_invalid_baseline() {
    assert_empty_baseline_invalid(
        ".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md",
        CanonicalArtifactKind::EnvironmentInventory,
        NextSafeAction::RunAuthorEnvironmentInventory,
    );
}

#[test]
fn doctor_keeps_all_starter_owned_baseline_in_scaffolded() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::Charter),
    );
    write_file(
        &repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::ProjectContext),
    );
    write_file(
        &repo_root.join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::EnvironmentInventory),
    );

    let report = doctor(repo_root).expect("doctor");
    assert_eq!(report.baseline_state, DoctorBaselineStatus::Scaffolded);
    assert_eq!(report.status, DoctorBaselineStatus::Scaffolded);
    assert_eq!(
        report.next_safe_action,
        Some(NextSafeAction::RunAuthorCharter)
    );
    assert!(report
        .checklist
        .iter()
        .all(|item| item.status == DoctorArtifactStatus::StarterOwned));
    assert_eq!(report.blockers.len(), 3);
    assert_eq!(
        report
            .blockers
            .iter()
            .map(|blocker| blocker.category)
            .collect::<Vec<_>>(),
        vec![
            BlockerCategory::RequiredArtifactStarterTemplate,
            BlockerCategory::RequiredArtifactStarterTemplate,
            BlockerCategory::RequiredArtifactStarterTemplate,
        ]
    );
}

#[test]
fn doctor_reports_root_missing_blocker_before_checklist_actions() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let report = doctor(repo_root).expect("doctor");

    assert_eq!(report.c04_result_version, C04_RESULT_VERSION);
    assert_eq!(report.c03_schema_version, C03_SCHEMA_VERSION);
    assert_eq!(
        report.c03_manifest_generation_version,
        MANIFEST_GENERATION_VERSION
    );
    assert_eq!(report.baseline_state, DoctorBaselineStatus::Scaffolded);
    assert_eq!(report.status, DoctorBaselineStatus::Scaffolded);
    assert_eq!(report.next_safe_action, Some(NextSafeAction::RunSetup));
    assert_eq!(report.blockers.len(), 1);
    assert_eq!(
        report.blockers[0].category,
        BlockerCategory::SystemRootMissing
    );
    assert_eq!(
        report.blockers[0].subject,
        SubjectRef::Policy {
            policy_id: "system_root"
        }
    );
}

#[test]
fn doctor_json_contract_serializes_exact_top_level_fields_for_missing_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    let report = doctor(dir.path()).expect("doctor");

    let value = assert_doctor_report_json_contract(&report);
    assert_eq!(value["baseline_state"], "scaffolded");
    assert_eq!(value["status"], "scaffolded");
    assert_eq!(value["system_root_status"], "missing");
    assert_eq!(
        value["next_safe_action"],
        serde_json::json!({ "kind": "run_setup" })
    );

    let blocker = &value["blockers"][0];
    assert_json_object_keys(
        blocker,
        &["category", "subject", "summary", "next_safe_action"],
    );
    assert_eq!(blocker["category"], "system_root_missing");
    assert_eq!(
        blocker["subject"],
        serde_json::json!({ "policy_id": "system_root" })
    );
    assert_eq!(
        blocker["next_safe_action"],
        serde_json::json!({ "kind": "run_setup" })
    );

    let checklist_item = &value["checklist"][0];
    assert_json_object_keys(
        checklist_item,
        &[
            "artifact_label",
            "subject",
            "author_command",
            "kind",
            "canonical_repo_relative_path",
            "status",
            "next_safe_action",
        ],
    );
    assert_eq!(checklist_item["artifact_label"], "CHARTER");
    assert_eq!(checklist_item["kind"], "charter");
    assert_eq!(checklist_item["status"], "missing");
    assert_eq!(
        checklist_item["subject"],
        serde_json::json!({
            "kind": "charter",
            "canonical_repo_relative_path": ".handbook/charter/CHARTER.md"
        })
    );
    assert_eq!(
        checklist_item["next_safe_action"],
        serde_json::json!({ "kind": "run_setup" })
    );
}

#[test]
fn doctor_reports_complete_baseline_with_empty_blockers() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        valid_environment_inventory_markdown().as_bytes(),
    );

    let report = doctor(repo_root).expect("doctor");

    assert_eq!(report.c04_result_version, C04_RESULT_VERSION);
    assert_eq!(report.c03_schema_version, C03_SCHEMA_VERSION);
    assert_eq!(
        report.c03_manifest_generation_version,
        MANIFEST_GENERATION_VERSION
    );
    assert_eq!(
        report.baseline_state,
        DoctorBaselineStatus::BaselineComplete
    );
    assert_eq!(report.status, DoctorBaselineStatus::BaselineComplete);
    assert!(report.blockers.is_empty());
    assert_eq!(report.next_safe_action, None);
}

#[test]
fn doctor_json_contract_preserves_non_ready_canonical_artifact_semantics() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::Charter),
    );
    write_file(
        &repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::ProjectContext),
    );
    write_file(
        &repo_root.join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::EnvironmentInventory),
    );

    let report = doctor(repo_root).expect("doctor");
    let value = assert_doctor_report_json_contract(&report);

    assert_eq!(value["baseline_state"], "scaffolded");
    assert_eq!(value["status"], "scaffolded");
    assert_eq!(
        value["next_safe_action"],
        serde_json::json!({ "kind": "run_author_charter" })
    );

    let checklist_item = value["checklist"]
        .as_array()
        .expect("checklist array")
        .iter()
        .find(|item| item["kind"] == "charter")
        .expect("charter checklist item");
    assert_eq!(checklist_item["status"], "starter_owned");
    assert_eq!(
        checklist_item["subject"],
        serde_json::json!({
            "kind": "charter",
            "canonical_repo_relative_path": ".handbook/charter/CHARTER.md"
        })
    );
    assert_eq!(
        checklist_item["next_safe_action"],
        serde_json::json!({ "kind": "run_author_charter" })
    );

    let blocker = value["blockers"]
        .as_array()
        .expect("blockers array")
        .iter()
        .find(|item| item["category"] == "required_artifact_starter_template")
        .expect("starter template blocker");
    assert_eq!(
        blocker["subject"],
        serde_json::json!({
            "kind": "charter",
            "canonical_repo_relative_path": ".handbook/charter/CHARTER.md"
        })
    );
    assert_eq!(
        blocker["next_safe_action"],
        serde_json::json!({ "kind": "run_author_charter" })
    );
}
