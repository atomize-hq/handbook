use handbook_engine::CanonicalArtifactKind;
#[cfg(unix)]
use handbook_engine::{
    parse_canonical_project_context, render_project_context_markdown,
    resolve_shipped_profile_decisions,
};
use handbook_flow::{
    resolve, resolve_with_contract, PacketSelectionStatus, ResolveRequest, ResolverNextSafeAction,
    ResolverRefusalCategory, ResolverSubjectRef,
};

#[path = "../../engine/tests/support/hcm_2_2_committed_charter.rs"]
mod hcm_2_2_committed_charter;
#[cfg(unix)]
use handbook_flow::{
    BudgetDisposition, BudgetPolicy, PacketSectionMode, PacketVariant, ReadyPacketNextSafeAction,
};

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
    concat!(
        "schema_id: \"handbook.artifact.project-context\"\n",
        "schema_version: \"1.0\"\n",
        "record_id: \"handbook.project-context\"\n",
        "summary: \"Project reality.\"\n",
        "system_boundaries:\n",
        "  - \"Canonical handbook truth\"\n",
        "ownership:\n",
        "  - \"handbook-team\"\n",
        "authoritative_references:\n",
        "  - \"handbook.charter@1.0.0\"\n",
        "known_unknowns:\n",
        "  - \"None\"\n",
    )
}

fn valid_environment_context_yaml() -> &'static str {
    concat!(
        "authoritative_references:\n",
        "  - \"handbook.project.environments@1.0.0\"\n",
        "environments:\n",
        "  -\n",
        "    capabilities:\n",
        "      - \"rust.stable\"\n",
        "      - \"filesystem.workspace-write\"\n",
        "    description: \"Local development.\"\n",
        "    environment_id: \"local-dev\"\n",
        "known_unknowns: []\n",
        "record_id: \"handbook.environment-context\"\n",
        "schema_id: \"handbook.artifact.environment-context\"\n",
        "schema_version: \"1.1\"\n",
    )
}

fn non_default_contract() -> handbook_engine::CanonicalLayoutContract {
    handbook_engine::CanonicalLayoutContract::from_paths(
        ".custom_handbook",
        ".custom_handbook/charter",
        ".custom_handbook/charter/CHARTER.md",
        ".custom_handbook/project_context",
        ".custom_handbook/project_context/PROJECT_CONTEXT.md",
        ".custom_handbook/feature_spec",
        ".custom_handbook/feature_spec/FEATURE_SPEC.md",
    )
}

#[cfg(unix)]
fn custom_handbook_path(relative: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(".custom_handbook").join(relative)
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
    assert_eq!(
        result
            .refusal
            .as_ref()
            .map(|refusal| refusal.summary.as_str()),
        Some("missing canonical root `.handbook`")
    );
}

#[test]
fn flow_resolver_blocks_missing_non_default_system_root_without_default_wording() {
    let dir = tempfile::tempdir().expect("tempdir");

    let result = resolve_with_contract(
        dir.path(),
        ResolveRequest::default(),
        non_default_contract(),
    )
    .expect("resolve");

    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, ResolverRefusalCategory::SystemRootMissing);
    assert_eq!(refusal.summary, "missing canonical root `.custom_handbook`");
    assert!(
        !refusal.summary.contains(".handbook"),
        "custom-contract system-root refusal should not fall back to default wording: {:?}",
        refusal.summary
    );
    assert!(result.blockers.iter().any(|blocker| {
        blocker.category == handbook_flow::ResolverBlockerCategory::SystemRootMissing
            && blocker.summary == "missing canonical root `.custom_handbook`"
            && !blocker.summary.contains(".handbook")
            && blocker.subject
                == ResolverSubjectRef::Policy {
                    policy_id: "system_root",
                }
            && blocker.next_safe_action == ResolverNextSafeAction::RunSetup
    }));
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

#[cfg(unix)]
#[test]
fn selected_project_context_alone_establishes_the_canonical_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("missing Charter refusal");

    assert_eq!(
        refusal.category,
        ResolverRefusalCategory::RequiredArtifactMissing
    );
    assert_eq!(
        refusal.broken_subject,
        ResolverSubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/charter/CHARTER.md".to_owned(),
        }
    );
    assert_eq!(
        refusal.next_safe_action,
        ResolverNextSafeAction::RunSetupRefresh
    );
    assert!(result
        .decision_log_entries
        .iter()
        .any(|entry| entry == "c03.handbook_root status=Ok"));
    assert!(result.blockers.iter().all(|blocker| {
        blocker.category != handbook_flow::ResolverBlockerCategory::SystemRootMissing
    }));
}

#[cfg(unix)]
#[test]
fn retired_project_context_alone_does_not_establish_the_canonical_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        b"retired editable Project Context truth",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("missing system root refusal");

    assert_eq!(refusal.category, ResolverRefusalCategory::SystemRootMissing);
    assert_eq!(refusal.next_safe_action, ResolverNextSafeAction::RunSetup);
    assert!(result
        .decision_log_entries
        .iter()
        .all(|entry| !entry.contains(".handbook/project_context/PROJECT_CONTEXT.md")));
}

#[cfg(unix)]
#[test]
fn flow_resolver_builds_ready_planning_packet_body() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
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
    assert_eq!(
        result.packet_result.sections[1].canonical_repo_relative_path,
        ".handbook/project/context.yaml"
    );
    assert_eq!(
        result.packet_result.sections[1].mode,
        PacketSectionMode::Rendered
    );
    assert!(result.packet_result.sections[1]
        .contents
        .starts_with("# Project Context\n"));
    assert_eq!(
        result.packet_result.sections[0].mode,
        PacketSectionMode::Verbatim
    );
    assert_eq!(
        result.packet_result.sections[0].contents,
        valid_charter_markdown()
    );
    assert_eq!(
        result.packet_result.decision_summary.ready_next_safe_action,
        ReadyPacketNextSafeAction::InspectProof
    );
}

#[cfg(unix)]
#[test]
fn flow_resolver_builds_ready_planning_packet_body_with_non_default_contract() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(custom_handbook_path("charter/CHARTER.md")),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(custom_handbook_path("project_context/PROJECT_CONTEXT.md")),
        b"conflicting legacy Project Context Markdown",
    );
    write_file(
        &root.join(custom_handbook_path("feature_spec/FEATURE_SPEC.md")),
        b"feature spec body",
    );

    let result = resolve_with_contract(root, ResolveRequest::default(), non_default_contract())
        .expect("resolve");

    assert!(result.packet_result.is_ready());
    assert_eq!(result.packet_result.variant, PacketVariant::Planning);
    assert_eq!(result.packet_result.included_sources.len(), 3);
    assert_eq!(
        result.packet_result.sections[0].canonical_repo_relative_path,
        ".custom_handbook/charter/CHARTER.md"
    );
    assert_eq!(
        result.packet_result.sections[1].canonical_repo_relative_path,
        ".handbook/project/context.yaml"
    );
    assert_eq!(
        result.packet_result.sections[2].canonical_repo_relative_path,
        ".custom_handbook/feature_spec/FEATURE_SPEC.md"
    );
}

#[cfg(unix)]
#[test]
fn flow_resolver_summarizes_optional_sources_when_budget_demands_it() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        "x".repeat(8192).as_bytes(),
    );

    let result = resolve(
        root,
        ResolveRequest {
            budget_policy: BudgetPolicy {
                max_total_bytes: None,
                max_per_artifact_bytes: Some(4096),
            },
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");

    assert_eq!(
        result.budget_outcome.disposition,
        BudgetDisposition::Summarize
    );
    let section = result
        .packet_result
        .sections
        .iter()
        .find(|section| section.title == "FEATURE_SPEC")
        .expect("feature spec section");
    assert_eq!(section.mode, PacketSectionMode::Summary);
    assert!(section
        .contents
        .contains("budget summary: full contents omitted"));
    assert!(result.packet_result.notes.iter().any(|note| {
        note.text
            == "optional source summarized due to budget: .handbook/feature_spec/FEATURE_SPEC.md (8192 bytes [source])"
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
            canonical_repo_relative_path: ".handbook/charter/CHARTER.md".to_owned(),
        }
    );
    assert_eq!(
        refusal.next_safe_action,
        ResolverNextSafeAction::RunSetupRefresh
    );
}

#[cfg(unix)]
#[test]
fn flow_resolver_never_opens_retired_project_context_non_regular_sentinel() {
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
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
    std::fs::create_dir_all(root.join(".handbook/project_context/PROJECT_CONTEXT.md"))
        .expect("project_context dir");

    let result = resolve(root, ResolveRequest::default()).expect("resolve");

    assert_eq!(result.selection.status, PacketSelectionStatus::Selected);
    assert!(result.refusal.is_none());
    assert!(!result.packet_result.sections.is_empty());
    assert!(
        !result.packet_result.notes.iter().any(|note| {
            note.text == "optional source omitted: .handbook/project_context/PROJECT_CONTEXT.md"
        }),
        "read errors must not be mislabeled as benign omissions: {:?}",
        result.packet_result.notes
    );
    assert!(result.blockers.is_empty());
    assert!(result
        .decision_log_entries
        .iter()
        .all(|entry| !entry.contains(".handbook/project_context/PROJECT_CONTEXT.md")));
}

#[cfg(unix)]
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
            canonical_repo_relative_path: ".handbook/charter/CHARTER.md".to_owned(),
        }
    );
    assert_eq!(
        refusal.next_safe_action,
        ResolverNextSafeAction::RunSetupRefresh
    );
}

#[cfg(unix)]
#[test]
fn flow_resolver_refuses_when_budget_is_exhausted() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
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

#[cfg(unix)]
#[test]
fn flow_resolver_budget_refusal_uses_non_default_contract_paths() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(custom_handbook_path("charter/CHARTER.md")),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(custom_handbook_path("feature_spec/FEATURE_SPEC.md")),
        b"feature spec that is longer than one byte",
    );

    let result = resolve_with_contract(
        root,
        ResolveRequest {
            budget_policy: BudgetPolicy {
                max_total_bytes: None,
                max_per_artifact_bytes: Some(1),
            },
            packet_id: "planning.packet",
        },
        non_default_contract(),
    )
    .expect("resolve");

    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, ResolverRefusalCategory::BudgetRefused);
    assert_eq!(
        refusal.next_safe_action,
        ResolverNextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path: ".custom_handbook/charter/CHARTER.md".to_owned(),
        }
    );
}

#[cfg(unix)]
#[test]
fn flow_resolver_refuses_live_execution_packets_without_fixture_backing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
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
    assert_eq!(
        refusal.category,
        ResolverRefusalCategory::UnsupportedRequest
    );
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

#[cfg(unix)]
#[test]
fn flow_resolver_builds_fixture_context_for_execution_demo_packets() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().join("tests/fixtures/execution_demo/basic");

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
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
    assert_eq!(fixture_context.fixture_lineage.len(), 3);
    assert_eq!(
        result.packet_result.decision_summary.ready_next_safe_action,
        ReadyPacketNextSafeAction::InspectProof
    );
}

#[cfg(unix)]
#[test]
fn flow_resolver_builds_honest_fixture_context_for_non_default_execution_demo_contracts() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().join("tests/fixtures/execution_demo/custom");

    write_file(
        &root.join(custom_handbook_path("charter/CHARTER.md")),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(custom_handbook_path("feature_spec/FEATURE_SPEC.md")),
        b"demo feature body",
    );

    let result = resolve_with_contract(
        &root,
        ResolveRequest {
            packet_id: "execution.demo.packet",
            ..ResolveRequest::default()
        },
        non_default_contract(),
    )
    .expect("resolve");

    assert!(result.packet_result.is_ready());
    let fixture_context = result
        .packet_result
        .fixture_context
        .as_ref()
        .expect("fixture context");
    assert_eq!(fixture_context.fixture_set_id, "custom");
    assert_eq!(
        fixture_context.fixture_basis_root,
        "tests/fixtures/execution_demo/custom/.custom_handbook/"
    );
    assert_eq!(fixture_context.fixture_lineage.len(), 3);
    assert_eq!(
        result.packet_result.sections[1].canonical_repo_relative_path,
        ".handbook/project/context.yaml"
    );
    assert!(result
        .packet_result
        .sections
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != 1)
        .all(|(_, section)| section
            .canonical_repo_relative_path
            .starts_with(".custom_handbook/")));
}

#[cfg(all(not(unix), not(windows)))]
#[test]
fn flow_resolver_refuses_selected_project_context_without_strict_read_support() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("selected Project Context refusal");
    assert_eq!(result.c04_result_version, "reduced-v1-m8.3");
    assert_eq!(
        refusal.category,
        ResolverRefusalCategory::RequiredArtifactInvalid
    );
    assert_eq!(
        refusal.broken_subject,
        ResolverSubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::ProjectContext,
            canonical_repo_relative_path: ".handbook/project/context.yaml".to_owned(),
        }
    );
    assert!(refusal.summary.contains("unsupported_platform_strict_read"));
    assert_eq!(
        refusal.next_safe_action,
        ResolverNextSafeAction::RunAuthorProjectContext
    );
}

const HCM_2_2_SELECTED_CHARTER_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

fn hcm_2_2_flow_fixture(root: &std::path::Path, legacy_charter: &[u8]) {
    hcm_2_2_committed_charter::promote_committed_charter(
        root,
        HCM_2_2_SELECTED_CHARTER_YAML.as_bytes(),
        "hcm-2-2-flow-fixture",
    );
    write_file(&root.join(".handbook/charter/CHARTER.md"), legacy_charter);
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature spec body",
    );
}

fn hcm_2_2_uncommitted_flow_fixture(root: &std::path::Path) {
    write_file(
        &root.join(".handbook/project/charter.yaml"),
        HCM_2_2_SELECTED_CHARTER_YAML.as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature spec body",
    );
}

#[test]
fn hcm_2_2_flow_projects_selected_charter_yaml_and_ignores_legacy_markdown() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    hcm_2_2_flow_fixture(root, valid_charter_markdown().as_bytes());

    let decisions =
        handbook_engine::resolve_shipped_profile_decisions(root).expect("selected profile");
    let selected = handbook_engine::load_selected_charter(root, &decisions)
        .expect("selected Charter projection");
    let first = resolve(root, ResolveRequest::default()).expect("first resolve");

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        b"legacy Charter Markdown must have zero selected influence\n",
    );
    let second = resolve(root, ResolveRequest::default()).expect("second resolve");

    assert_eq!(first.c04_result_version, "reduced-v1-m8.3");
    assert_eq!(first.c03_schema_version, "reduced-v1-m8");
    assert_eq!(first.c03_manifest_generation_version, 1);
    assert_eq!(first, second, "legacy Charter Markdown changed flow truth");
    assert_eq!(first.selection.status, PacketSelectionStatus::Selected);

    let source = first
        .packet_result
        .included_sources
        .iter()
        .find(|source| source.kind == CanonicalArtifactKind::Charter)
        .expect("selected Charter source");
    assert_eq!(
        source.canonical_repo_relative_path,
        selected.canonical_path()
    );
    assert_eq!(
        source.content_sha256.as_deref(),
        Some(
            selected
                .source_fingerprint()
                .as_str()
                .strip_prefix("sha256:")
                .expect("sha256 domain")
        )
    );
    assert_eq!(
        source.rendered_output_sha256.as_deref(),
        Some(selected.rendered_output_fingerprint().as_str())
    );
    assert_eq!(
        source.rendered_output_byte_len,
        Some(selected.rendered_byte_length() as u64)
    );
    assert_eq!(source.rendered_media_type.as_deref(), Some("text/markdown"));

    let section = first
        .packet_result
        .sections
        .iter()
        .find(|section| section.kind == CanonicalArtifactKind::Charter)
        .expect("rendered Charter section");
    assert_eq!(section.mode, handbook_flow::PacketSectionMode::Rendered);
    assert_eq!(
        section.canonical_repo_relative_path,
        selected.canonical_path()
    );
    assert_eq!(section.contents.as_bytes(), selected.rendered_bytes());
    assert_eq!(
        section.source_content_sha256.as_deref(),
        Some(selected.source_fingerprint().as_str())
    );
    assert_eq!(
        section.rendered_output_sha256.as_deref(),
        Some(selected.rendered_output_fingerprint().as_str())
    );
    assert!(first.decision_log_entries.iter().all(|entry| {
        !entry.contains(".handbook/charter/CHARTER.md")
            && !entry.contains("legacy Charter Markdown")
    }));
    assert!(first.decision_log_entries.iter().any(|entry| {
        entry.contains("bridge=BR-HCM-2-CHARTER-FLOW-01")
            && entry.contains("promotion_ref=promotions/")
            && entry.contains("lifecycle_transition_ref=lifecycle-transitions/")
    }));
}

#[test]
fn valid_environment_context_is_rendered_into_the_advisory_packet() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    hcm_2_2_flow_fixture(root, valid_charter_markdown().as_bytes());
    write_file(
        &root.join(".handbook/project/environment.yaml"),
        valid_environment_context_yaml().as_bytes(),
    );

    let decisions =
        handbook_engine::resolve_shipped_profile_decisions(root).expect("selected profile");
    let selected = handbook_engine::load_selected_environment_context(root, &decisions)
        .expect("selected Environment Context projection");
    let result = resolve(root, ResolveRequest::default()).expect("resolve");

    assert!(result.packet_result.is_ready());
    assert!(result.refusal.is_none());
    assert!(result.blockers.is_empty());
    let source = result
        .packet_result
        .included_sources
        .iter()
        .find(|source| source.kind == CanonicalArtifactKind::EnvironmentContext)
        .expect("advisory Environment Context source");
    assert_eq!(
        source.canonical_repo_relative_path,
        selected.canonical_path()
    );
    assert!(!source.required);
    assert_eq!(
        source.rendered_output_sha256.as_deref(),
        Some(selected.rendered_output_fingerprint().as_str())
    );
    let section = result
        .packet_result
        .sections
        .iter()
        .find(|section| section.kind == CanonicalArtifactKind::EnvironmentContext)
        .expect("advisory Environment Context section");
    assert_eq!(section.mode, handbook_flow::PacketSectionMode::Rendered);
    assert_eq!(section.contents.as_bytes(), selected.rendered_bytes());
}

#[test]
fn oversized_environment_context_is_summarized_without_rendered_body() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    hcm_2_2_flow_fixture(root, valid_charter_markdown().as_bytes());
    let oversized_description = "x".repeat(8_000);
    let yaml = valid_environment_context_yaml().replace(
        "description: \"Local development.\"",
        format!("description: \"{oversized_description}\"").as_str(),
    );
    write_file(
        &root.join(".handbook/project/environment.yaml"),
        yaml.as_bytes(),
    );

    let decisions =
        handbook_engine::resolve_shipped_profile_decisions(root).expect("selected profile");
    let selected = handbook_engine::load_selected_environment_context(root, &decisions)
        .expect("selected Environment Context projection");
    let result = resolve(
        root,
        ResolveRequest {
            budget_policy: handbook_flow::BudgetPolicy {
                max_total_bytes: None,
                max_per_artifact_bytes: Some(selected.rendered_bytes().len() as u64 - 1),
            },
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");

    assert_eq!(
        result.budget_outcome.disposition,
        handbook_flow::BudgetDisposition::Summarize
    );
    let source = result
        .packet_result
        .included_sources
        .iter()
        .find(|source| source.kind == CanonicalArtifactKind::EnvironmentContext)
        .expect("summarized Environment Context source");
    assert!(source.rendered_output_byte_len.is_none());
    assert!(source.rendered_output_sha256.is_none());
    let section = result
        .packet_result
        .sections
        .iter()
        .find(|section| section.kind == CanonicalArtifactKind::EnvironmentContext)
        .expect("summarized Environment Context section");
    assert_eq!(section.mode, handbook_flow::PacketSectionMode::Summary);
    assert!(!section.contents.contains(oversized_description.as_str()));
    assert!(result.packet_result.notes.iter().any(|note| {
        note.text.starts_with(
            "optional source summarized due to budget: .handbook/project/environment.yaml",
        )
    }));
}

#[test]
fn invalid_environment_context_is_reported_without_blocking_the_packet() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    hcm_2_2_flow_fixture(root, valid_charter_markdown().as_bytes());
    write_file(
        &root.join(".handbook/project/environment.yaml"),
        b"schema_id: handbook.artifact.environment-context\nschema_version: '1.1'\n",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");

    assert!(result.packet_result.is_ready());
    assert!(result.refusal.is_none());
    assert!(result.blockers.is_empty());
    assert!(result
        .packet_result
        .sections
        .iter()
        .all(|section| { section.kind != CanonicalArtifactKind::EnvironmentContext }));
    assert!(result.packet_result.notes.iter().any(|note| {
        note.text
            == "optional source omitted: .handbook/project/environment.yaml (invalid canonical truth)"
    }));
}

#[test]
fn hcm_2_2_flow_refuses_selected_charter_without_committed_current_authority() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    hcm_2_2_uncommitted_flow_fixture(root);

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("uncommitted Charter refusal");

    assert_eq!(
        refusal.category,
        ResolverRefusalCategory::RequiredArtifactInvalid
    );
    assert_eq!(
        refusal.broken_subject,
        ResolverSubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/project/charter.yaml".to_owned(),
        }
    );
}

#[test]
fn hcm_2_2_flow_requires_selected_charter_even_when_legacy_markdown_is_valid() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    hcm_2_2_flow_fixture(root, valid_charter_markdown().as_bytes());
    std::fs::remove_file(root.join(".handbook/project/charter.yaml"))
        .expect("remove selected Charter");

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("selected Charter refusal");

    assert_eq!(
        refusal.category,
        ResolverRefusalCategory::RequiredArtifactInvalid
    );
    assert_eq!(
        refusal.broken_subject,
        ResolverSubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/project/charter.yaml".to_owned(),
        }
    );
}

#[test]
fn hcm_2_2_flow_budgets_selected_charter_in_the_rendered_domain() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    hcm_2_2_flow_fixture(root, valid_charter_markdown().as_bytes());

    let decisions =
        handbook_engine::resolve_shipped_profile_decisions(root).expect("selected profile");
    let selected = handbook_engine::load_selected_charter(root, &decisions)
        .expect("selected Charter projection");
    let result = resolve(
        root,
        ResolveRequest {
            budget_policy: handbook_flow::BudgetPolicy {
                max_total_bytes: None,
                max_per_artifact_bytes: Some(selected.rendered_byte_length() as u64 - 1),
            },
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");

    let target = result
        .budget_outcome
        .targets
        .iter()
        .find(|target| target.canonical_repo_relative_path == selected.canonical_path())
        .expect("selected Charter budget target");
    assert_eq!(target.byte_len, selected.rendered_byte_length() as u64);
    assert_eq!(
        target.byte_domain,
        handbook_flow::BudgetByteDomain::RenderedOutput
    );
}
