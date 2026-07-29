use handbook_compiler::resolve;
#[cfg(unix)]
use handbook_compiler::{render_next_safe_action_value, BlockerCategory};
#[cfg(unix)]
use handbook_engine::{
    parse_canonical_project_context, render_project_context_markdown,
    resolve_shipped_profile_decisions,
};
#[cfg(unix)]
use handbook_engine::{setup_starter_template_bytes, CanonicalArtifactKind};
#[cfg(unix)]
use handbook_flow::{
    BudgetDisposition, BudgetPolicy, PacketSectionMode, PacketVariant, ReadyPacketNextSafeAction,
};
use handbook_flow::{PacketSelectionStatus, ResolveRequest};

#[cfg(unix)]
fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[cfg(unix)]
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

#[cfg(unix)]
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

#[cfg(unix)]
fn invalid_optional_project_context_markdown() -> String {
    format!(
        "{}unexpected_field: true\n",
        valid_project_context_markdown()
    )
}

#[cfg(unix)]
fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};

    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(unix)]
fn write_valid_project_context(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
}

#[cfg(unix)]
fn required_budget_bytes(repo_root: &std::path::Path) -> u64 {
    let decisions = resolve_shipped_profile_decisions(repo_root).expect("shipped decisions");
    let project_context =
        parse_canonical_project_context(&decisions, valid_project_context_markdown().as_bytes())
            .expect("canonical Project Context");
    valid_charter_markdown().len() as u64
        + render_project_context_markdown(&project_context)
            .expect("rendered Project Context")
            .len() as u64
}

#[test]
fn resolver_returns_typed_result_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let result = resolve(repo_root, ResolveRequest::default()).expect("resolve");

    assert_eq!(result.c04_result_version, "reduced-v1-m8.3");
    assert_eq!(result.c03_schema_version, "reduced-v1-m8");
    assert_eq!(result.c03_manifest_generation_version, 1);
    assert_eq!(result.selection.status, PacketSelectionStatus::Blocked);
    assert!(result.packet_result.sections.is_empty());
    assert!(result
        .packet_result
        .notes
        .iter()
        .any(|note| { note.text == "packet body omitted because request is not ready" }));
    assert_eq!(result.c03_fingerprint_sha256.len(), 64);
    assert!(result
        .c03_fingerprint_sha256
        .chars()
        .all(|c| c.is_ascii_hexdigit()));
}

#[cfg(unix)]
#[test]
fn optional_artifact_read_error_blocks_without_refusal() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    write_valid_project_context(repo_root);
    std::fs::create_dir_all(repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"))
        .expect("project_context dir");

    let result = resolve(repo_root, ResolveRequest::default()).expect("resolve");

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
}

#[cfg(unix)]
#[test]
fn missing_optional_project_context_emits_omission_note() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(repo_root, ResolveRequest::default()).expect("resolve");

    assert_eq!(result.selection.status, PacketSelectionStatus::Blocked);
    let refusal = result
        .refusal
        .expect("required selected Project Context refusal");
    assert_eq!(
        refusal.category,
        handbook_compiler::RefusalCategory::RequiredArtifactInvalid
    );
    assert!(refusal.summary.contains("required_path_missing"));
}

#[cfg(unix)]
#[test]
fn semantically_invalid_optional_project_context_is_omitted_from_ready_packet() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/project/context.yaml"),
        invalid_optional_project_context_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(repo_root, ResolveRequest::default()).expect("resolve");

    assert_eq!(result.selection.status, PacketSelectionStatus::Blocked);
    let refusal = result
        .refusal
        .expect("invalid selected Project Context refusal");
    assert_eq!(
        refusal.category,
        handbook_compiler::RefusalCategory::RequiredArtifactInvalid
    );
}

#[cfg(unix)]
#[test]
fn semantically_invalid_required_charter_blocks_with_required_artifact_invalid() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(&repo_root.join(".handbook/charter/CHARTER.md"), b"charter");
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(repo_root, ResolveRequest::default()).expect("resolve");

    assert_eq!(result.selection.status, PacketSelectionStatus::Blocked);
    let refusal = result.refusal.expect("refusal");
    assert_eq!(
        refusal.category,
        handbook_compiler::RefusalCategory::RequiredArtifactInvalid
    );
    assert!(result
        .blockers
        .iter()
        .any(|blocker| blocker.category == BlockerCategory::RequiredArtifactInvalid));
}

#[cfg(unix)]
#[test]
fn required_starter_template_blocks_without_ready_packet() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::Charter),
    );
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    write_file(
        &repo_root.join(".handbook/project/context.yaml"),
        setup_starter_template_bytes(CanonicalArtifactKind::ProjectContext),
    );

    let result = resolve(repo_root, ResolveRequest::default()).expect("resolve");

    assert_eq!(result.selection.status, PacketSelectionStatus::Blocked);
    let refusal = result.refusal.expect("refusal");
    assert_eq!(
        refusal.category,
        handbook_compiler::RefusalCategory::RequiredArtifactStarterTemplate
    );
    assert_eq!(
        render_next_safe_action_value(&refusal.next_safe_action),
        "run `handbook author charter --from-inputs <path|->`"
    );
    assert!(result.blockers.iter().any(|blocker| blocker.category
        == BlockerCategory::RequiredArtifactStarterTemplate
        && render_next_safe_action_value(&blocker.next_safe_action)
            == "run `handbook author charter --from-inputs <path|->`"));
    assert!(result.packet_result.sections.is_empty());
}

#[cfg(unix)]
#[test]
fn resolver_is_deterministic_for_identical_inputs() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        "f".repeat(4096).as_bytes(),
    );
    write_valid_project_context(repo_root);

    let req = ResolveRequest::default();
    let a = resolve(repo_root, req.clone()).expect("resolve a");
    let b = resolve(repo_root, req).expect("resolve b");

    assert_eq!(a, b);
}

#[cfg(unix)]
#[test]
fn budget_next_safe_action_is_only_present_on_refuse() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        "f".repeat(4096).as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );

    // Summarize optional.
    let summarize_req = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: None,
            max_per_artifact_bytes: Some(valid_charter_markdown().len() as u64),
        },
        ..ResolveRequest::default()
    };
    let summarize = resolve(repo_root, summarize_req).expect("resolve summarize");
    assert_eq!(
        summarize.budget_outcome.disposition,
        BudgetDisposition::Summarize
    );
    assert!(summarize.budget_outcome.next_safe_action.is_none());

    // Exclude optional.
    let exclude_req = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: Some(required_budget_bytes(repo_root)),
            max_per_artifact_bytes: None,
        },
        ..ResolveRequest::default()
    };
    let exclude = resolve(repo_root, exclude_req).expect("resolve exclude");
    assert_eq!(
        exclude.budget_outcome.disposition,
        BudgetDisposition::Exclude
    );
    assert!(exclude.budget_outcome.next_safe_action.is_none());

    // Refuse required.
    let refuse_req = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: None,
            max_per_artifact_bytes: Some(0),
        },
        ..ResolveRequest::default()
    };
    let refuse = resolve(repo_root, refuse_req).expect("resolve refuse");
    assert_eq!(refuse.budget_outcome.disposition, BudgetDisposition::Refuse);
    assert!(refuse.budget_outcome.next_safe_action.is_some());
}

#[cfg(unix)]
#[test]
fn budget_summarize_replaces_optional_body_with_summary() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        "feature".repeat(1024).as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );

    let summarize_req = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: None,
            max_per_artifact_bytes: Some(valid_charter_markdown().len() as u64),
        },
        ..ResolveRequest::default()
    };
    let result = resolve(repo_root, summarize_req).expect("resolve summarize");

    assert_eq!(
        result.budget_outcome.disposition,
        BudgetDisposition::Summarize
    );
    assert_eq!(result.packet_result.included_sources.len(), 3);
    let summarized_section = result
        .packet_result
        .sections
        .iter()
        .find(|section| section.title == "FEATURE_SPEC")
        .expect("feature spec section");
    assert_eq!(summarized_section.mode, PacketSectionMode::Summary);
    assert!(
        summarized_section
            .contents
            .contains("budget summary: full contents omitted"),
        "expected budget summary stub: {:?}",
        summarized_section.contents
    );
    assert!(
        !summarized_section
            .contents
            .contains("featurefeaturefeature"),
        "full optional contents should not leak once summarized: {:?}",
        summarized_section.contents
    );
}

#[cfg(unix)]
#[test]
fn budget_exclude_removes_optional_body_from_packet() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    write_file(
        &repo_root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );

    let exclude_req = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: Some(required_budget_bytes(repo_root)),
            max_per_artifact_bytes: None,
        },
        ..ResolveRequest::default()
    };
    let result = resolve(repo_root, exclude_req).expect("resolve exclude");

    assert_eq!(
        result.budget_outcome.disposition,
        BudgetDisposition::Exclude
    );
    assert_eq!(result.packet_result.included_sources.len(), 2);
    assert!(
        result
            .packet_result
            .included_sources
            .iter()
            .all(|source| source.canonical_repo_relative_path
                != ".handbook/feature_spec/FEATURE_SPEC.md"),
        "excluded sources should not be listed as included: {:?}",
        result.packet_result.included_sources
    );
    assert_eq!(result.packet_result.sections.len(), 2);
    assert!(
        result
            .packet_result
            .sections
            .iter()
            .all(|section| section.title != "FEATURE_SPEC"),
        "excluded optional section should be absent from packet body: {:?}",
        result.packet_result.sections
    );
}

#[cfg(unix)]
#[test]
fn resolver_builds_typed_packet_body_for_planning_packet() {
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
    assert!(
        result
            .packet_result
            .decision_summary
            .summary_line
            .contains("READY planning.packet"),
        "expected ready summary line: {:?}",
        result.packet_result.decision_summary.summary_line
    );
}

#[cfg(unix)]
#[test]
fn ready_packet_sections_match_included_source_metadata() {
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

    for section in result
        .packet_result
        .sections
        .iter()
        .filter(|section| section.mode == PacketSectionMode::Verbatim)
    {
        let source = result
            .packet_result
            .included_sources
            .iter()
            .find(|source| {
                source.canonical_repo_relative_path == section.canonical_repo_relative_path
            })
            .expect("matching included source");

        let bytes = section.contents.as_bytes();
        assert_eq!(source.byte_len, Some(bytes.len() as u64));
        assert_eq!(
            source.content_sha256.as_deref(),
            Some(sha256_hex(bytes).as_str())
        );
    }
}

#[cfg(unix)]
#[test]
fn resolver_builds_fixture_context_for_execution_demo_packets() {
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

    let request = ResolveRequest {
        packet_id: "execution.demo.packet",
        ..ResolveRequest::default()
    };

    let result = resolve(&root, request).expect("resolve");

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
        fixture_context.fixture_lineage[0].canonical_repo_relative_path,
        ".handbook/charter/CHARTER.md"
    );
    assert_eq!(
        fixture_context.fixture_lineage[1].canonical_repo_relative_path,
        ".handbook/project/context.yaml"
    );
    assert_eq!(
        fixture_context.fixture_lineage[2].canonical_repo_relative_path,
        ".handbook/feature_spec/FEATURE_SPEC.md"
    );
    assert_eq!(
        result.packet_result.decision_summary.ready_next_safe_action,
        ReadyPacketNextSafeAction::InspectProof
    );
}

#[cfg(unix)]
#[test]
fn resolver_redacts_packet_body_for_unsupported_live_execution_requests() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature body",
    );
    write_valid_project_context(root);

    let result = resolve(
        root,
        ResolveRequest {
            packet_id: "execution.live.packet",
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");

    assert_eq!(result.selection.status, PacketSelectionStatus::Blocked);
    assert!(result.refusal.is_some());
    assert!(result.packet_result.sections.is_empty());
    assert!(result
        .packet_result
        .notes
        .iter()
        .any(|note| { note.text == "packet body omitted because request is not ready" }));
}
