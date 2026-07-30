use handbook_compiler::{render_next_safe_action_value, resolve, RefusalCategory, SubjectRef};
use handbook_engine::{setup_starter_template_bytes, CanonicalArtifactKind};
use handbook_flow::ResolveRequest;
#[cfg(unix)]
use handbook_flow::{BudgetDisposition, BudgetPolicy};

#[cfg(unix)]
#[path = "../../engine/tests/support/hcm_2_2_committed_charter.rs"]
mod hcm_2_2_committed_charter;

#[cfg(unix)]
const HCM_2_2_SELECTED_CHARTER_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
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

    write_file(
        &root.join(".handbook/project/context.yaml"),
        b"project context",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::RequiredArtifactInvalid);
    assert_eq!(
        refusal.broken_subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/project/charter.yaml".to_owned(),
        }
    );
}

#[cfg(unix)]
#[test]
fn refusal_non_canonical_input_attempt_is_selected_for_symlinked_canonical_artifact() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    std::fs::create_dir_all(root.join(".handbook/project")).expect("mkdirs");

    let real = root.join("real_charter.yaml");
    write_file(&real, b"charter");
    symlink(&real, root.join(".handbook/project/charter.yaml")).expect("symlink charter");
    write_file(
        &root.join(".handbook/project/context.yaml"),
        b"project context",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::NonCanonicalInputAttempt);
    assert_eq!(
        refusal.broken_subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/project/charter.yaml".to_owned(),
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

    write_file(&root.join(".handbook/project/charter.yaml"), b"");
    write_file(
        &root.join(".handbook/project/context.yaml"),
        b"project context",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::RequiredArtifactInvalid);
    assert_eq!(
        refusal.broken_subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/project/charter.yaml".to_owned(),
        }
    );
}

#[test]
fn refusal_required_artifact_starter_template() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/project/charter.yaml"),
        setup_starter_template_bytes(CanonicalArtifactKind::Charter),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        b"project context",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::RequiredArtifactInvalid);
    assert_eq!(
        refusal.broken_subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/project/charter.yaml".to_owned(),
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

    write_file(&root.join(".handbook/project/charter.yaml"), b"charter");
    write_file(
        &root.join(".handbook/project/context.yaml"),
        b"project context",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::RequiredArtifactInvalid);
    assert_eq!(
        refusal.broken_subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/project/charter.yaml".to_owned(),
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

    std::fs::create_dir_all(root.join(".handbook/project/charter.yaml")).expect("charter dir");
    write_file(
        &root.join(".handbook/project/context.yaml"),
        b"project context",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::ArtifactReadError);
    assert_eq!(
        refusal.broken_subject,
        SubjectRef::CanonicalArtifact {
            kind: CanonicalArtifactKind::Charter,
            canonical_repo_relative_path: ".handbook/project/charter.yaml".to_owned(),
        }
    );
    assert_eq!(
        render_next_safe_action_value(&refusal.next_safe_action),
        "run `handbook setup refresh`"
    );
}

#[cfg(unix)]
#[test]
fn refusal_budget_refused_is_selected_when_other_inputs_ok() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    hcm_2_2_committed_charter::promote_committed_charter(
        root,
        HCM_2_2_SELECTED_CHARTER_YAML.as_bytes(),
        "compiler-refusal-budget",
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        include_bytes!("../../../tools/fixtures/project_context_inputs/runtime_smoke_valid.yaml"),
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

#[cfg(unix)]
#[test]
fn refusal_unsupported_request_is_selected_for_live_execution_packet_when_other_inputs_ok() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    hcm_2_2_committed_charter::promote_committed_charter(
        root,
        HCM_2_2_SELECTED_CHARTER_YAML.as_bytes(),
        "compiler-refusal-unsupported",
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        include_bytes!("../../../tools/fixtures/project_context_inputs/runtime_smoke_valid.yaml"),
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
