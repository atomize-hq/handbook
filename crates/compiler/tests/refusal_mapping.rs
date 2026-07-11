use handbook_compiler::{render_next_safe_action_value, resolve, RefusalCategory, SubjectRef};
use handbook_engine::{setup_starter_template_bytes, CanonicalArtifactKind};
use handbook_flow::{BudgetDisposition, BudgetPolicy, ResolveRequest};

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

#[test]
fn refusal_system_root_missing_is_highest_priority() {
    let dir = tempfile::tempdir().expect("tempdir");
    let result = resolve(dir.path(), ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::SystemRootMissing);
}

#[test]
fn refusal_required_artifact_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    std::fs::create_dir_all(root.join(".handbook/feature_spec")).expect("mkdirs");
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::RequiredArtifactMissing);
}

#[cfg(unix)]
#[test]
fn refusal_non_canonical_input_attempt_is_selected_for_symlinked_canonical_artifact() {
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
    assert_eq!(refusal.category, RefusalCategory::NonCanonicalInputAttempt);
    assert_eq!(
        refusal.broken_subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/charter/CHARTER.md",
        }
    );
    assert_eq!(
        render_next_safe_action_value(&refusal.next_safe_action),
        "run `handbook setup refresh`"
    );
}

#[test]
fn refusal_required_artifact_empty() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".handbook/charter/CHARTER.md"), b"");
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::RequiredArtifactEmpty);
}

#[test]
fn refusal_required_artifact_starter_template() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::Charter),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(
        refusal.category,
        RefusalCategory::RequiredArtifactStarterTemplate
    );
    assert_eq!(
        refusal.broken_subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/charter/CHARTER.md",
        }
    );
    assert_eq!(
        render_next_safe_action_value(&refusal.next_safe_action),
        "run `handbook author charter --from-inputs <path|->`"
    );
}

#[test]
fn refusal_required_artifact_invalid() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".handbook/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::RequiredArtifactInvalid);
    assert_eq!(
        refusal.broken_subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/charter/CHARTER.md",
        }
    );
    assert_eq!(
        render_next_safe_action_value(&refusal.next_safe_action),
        "run `handbook author charter --from-inputs <path|->`"
    );
}

#[test]
fn refusal_required_artifact_read_error_is_selected_for_malformed_required_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    std::fs::create_dir_all(root.join(".handbook/charter/CHARTER.md")).expect("charter dir");
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::ArtifactReadError);
    assert_eq!(
        refusal.broken_subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/charter/CHARTER.md",
        }
    );
    assert_eq!(
        render_next_safe_action_value(&refusal.next_safe_action),
        "run `handbook setup refresh`"
    );
}

#[test]
fn refusal_budget_refused_is_selected_when_other_inputs_ok() {
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

    let request = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: None,
            max_per_artifact_bytes: Some(1),
        },
        packet_id: "planning.packet",
    };

    let result = resolve(root, request).expect("resolve");
    assert_eq!(result.budget_outcome.disposition, BudgetDisposition::Refuse);
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::BudgetRefused);
}

#[test]
fn refusal_unsupported_request_is_selected_for_live_execution_packet_when_other_inputs_ok() {
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

    let request = ResolveRequest {
        budget_policy: BudgetPolicy::default(),
        packet_id: "execution.live.packet",
    };

    let result = resolve(root, request).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::UnsupportedRequest);
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
