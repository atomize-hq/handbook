use handbook_compiler::{
    doctor, execute_charter_command, parse_charter_intake_envelope, run_setup,
    AdapterOperationStatus, CharterCommandIntent, DoctorCharterDefinitionClosureStatus,
    DoctorCharterLifecycleState, DoctorCharterNextAction, SetupMode, SetupRequest,
    DOCTOR_REPORT_SCHEMA_VERSION,
};
use handbook_engine::{
    evaluate_charter_intake, resolve_shipped_profile_decisions, ArtifactInspectionReason,
    ArtifactInspectionStatus, CharterAcquisitionMode,
};

const CHARTER_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

const COVERAGE_IDS: [&str; 16] = [
    "project_shape.definition",
    "delivery.constraints",
    "delivery.default_implications",
    "operational_reality.production_state",
    "risk.domains",
    "engineering_posture.baseline",
    "policy.authority_and_revision",
    "governance.decision_authority",
    "governance.required_approvals",
    "governance.exception_policy",
    "engineering_posture.dimensions",
    "engineering_posture.red_lines",
    "governance.review_triggers",
    "governance.reassessment_triggers",
    "debt.register",
    "decisions.records",
];

fn valid_intake_yaml() -> String {
    let mut document = String::from("mode: guided_adaptive\ncontent:\n");
    for line in CHARTER_YAML.lines() {
        document.push_str("  ");
        document.push_str(line);
        document.push('\n');
    }
    document.push_str("coverage:\n");
    for coverage_id in COVERAGE_IDS {
        document.push_str(&format!(
            "  - coverage_id: {coverage_id:?}\n    source_kind: user_declaration\n    value_ref: \"input://{coverage_id}\"\n    evidence_refs: []\n    confidence: high\n    freshness: null\n    sensitivity: public\n    contradiction_refs: []\n    waiver_ref: null\n"
        ));
    }
    document.push_str(
        "consumer:\n  kind: agent\n  id: handbook-charter-intake\n  version: \"1.0\"\nprompt_event_refs: []\nfinalized_at_utc: \"2026-07-20T00:00:00Z\"\nexpected_current_fingerprint: null\n",
    );
    document
}

#[test]
fn author_persists_exact_immutable_bundle_without_selecting_canonical_truth() {
    let repo = tempfile::tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
    let envelope = parse_charter_intake_envelope(&valid_intake_yaml()).unwrap();
    let expected = evaluate_charter_intake(&decisions, envelope.clone(), None).unwrap();
    let result = execute_charter_command(
        repo.path(),
        CharterCommandIntent::Author {
            mode: CharterAcquisitionMode::GuidedAdaptive,
            envelope,
        },
    );

    assert_eq!(result.status, AdapterOperationStatus::Succeeded);
    assert_eq!(
        result.intake_ref.as_deref(),
        Some(expected.candidate_subject.intake_record_ref.as_str())
    );
    assert_eq!(
        result.intake_fingerprint.as_deref(),
        Some(expected.intake.record_fingerprint.as_str())
    );
    let candidate_ref = result.candidate_ref.as_deref().expect("candidate ref");
    let candidate: serde_json::Value = serde_json::from_slice(
        &std::fs::read(
            repo.path()
                .join(".handbook/evidence/charter")
                .join(candidate_ref),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(candidate["schema_version"], "1.3");
    assert_eq!(
        candidate["candidate_subject_fingerprint"],
        expected.candidate_subject.candidate_subject_fingerprint
    );
    assert_eq!(
        candidate["candidate_fingerprint"].as_str(),
        result.candidate_fingerprint.as_deref()
    );
    let validation_result_ref = candidate["validation_result_binding"]["validation_result_ref"]
        .as_str()
        .expect("validation-result ref");
    assert_eq!(
        result.changed_paths,
        vec![
            format!(
                ".handbook/state/{}",
                expected.candidate_subject.normalized_content_ref
            ),
            format!(
                ".handbook/state/{}",
                expected.candidate_subject.intake_record_ref
            ),
            format!(".handbook/state/{validation_result_ref}"),
            format!(".handbook/evidence/charter/{candidate_ref}"),
        ]
    );
    assert!(result.refusal.is_none());
    for path in &result.changed_paths {
        assert!(repo.path().join(path).is_file());
    }
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
}

#[test]
fn author_persistence_collision_refuses_without_publishing_semantic_refs() {
    let repo = tempfile::tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
    let envelope = parse_charter_intake_envelope(&valid_intake_yaml()).unwrap();
    let expected = evaluate_charter_intake(&decisions, envelope.clone(), None).unwrap();
    let collision = repo
        .path()
        .join(".handbook/state")
        .join(&expected.candidate_subject.normalized_content_ref);
    std::fs::create_dir_all(collision.parent().unwrap()).unwrap();
    std::fs::write(&collision, b"unequal existing candidate content").unwrap();

    let result = execute_charter_command(
        repo.path(),
        CharterCommandIntent::Author {
            mode: CharterAcquisitionMode::GuidedAdaptive,
            envelope,
        },
    );

    assert_eq!(result.status, AdapterOperationStatus::Refused);
    assert_eq!(
        result.refusal.as_ref().map(|refusal| refusal.code.as_str()),
        Some("lineage_persistence_refused")
    );
    assert!(result.intake_ref.is_none());
    assert!(result.intake_fingerprint.is_none());
    assert!(result.candidate_ref.is_none());
    assert!(result.candidate_fingerprint.is_none());
    assert!(result.changed_paths.is_empty());
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
    assert!(!repo
        .path()
        .join(".handbook/state")
        .join(&expected.candidate_subject.intake_record_ref)
        .exists());
}

#[test]
fn approval_projects_exact_repository_identity_preflight_before_native_authority() {
    let repo = tempfile::tempdir().unwrap();

    let result = execute_charter_command(
        repo.path(),
        CharterCommandIntent::Approve {
            candidate_ref: "candidates/candidate_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json".to_owned(),
            approval_class: "Project owner approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
            accepted_waiver_refs: Vec::new(),
        },
    );
    let value = serde_json::to_value(result).unwrap();

    assert_eq!(
        value,
        serde_json::json!({
            "schema_id": "handbook.repository-invocation-preflight-result",
            "schema_version": "1.0",
            "operation": "charter_approval",
            "stage": "repository_identity",
            "status": "refused",
            "repository_identity_fingerprint": null,
            "operation_id": null,
            "changed_paths": [],
            "refusal": {
                "code": "repository_identity_unavailable",
                "message": "repository invocation identity is unavailable",
                "retryable": false
            },
            "next_actions": [
                "run or repair handbook setup, then retry the complete operation"
            ]
        })
    );
    assert!(!repo.path().join(".handbook").exists());
}

#[test]
fn promotion_projects_the_engine_owned_invalid_intent_refusal() {
    let repo = tempfile::tempdir().unwrap();

    let result = execute_charter_command(
        repo.path(),
        CharterCommandIntent::Promote {
            candidate_ref: String::new(),
            approval_ref: String::new(),
            expected_current_fingerprint: None,
        },
    );

    assert_eq!(result.status, AdapterOperationStatus::Refused);
    assert!(result.changed_paths.is_empty());
    assert_eq!(
        result.refusal.as_ref().map(|refusal| (
            refusal.code.as_str(),
            refusal.message.as_str(),
            refusal.retryable,
        )),
        Some((
            "invalid_intent",
            "promotion candidate or approval anchor ref is invalid",
            false,
        ))
    );
    assert_eq!(
        result.next_actions,
        vec!["correct the bounded promotion intent and retry"]
    );
    assert!(!repo.path().join(".handbook").exists());
}

#[test]
fn setup_creates_only_the_root_and_never_authors_selected_charter_truth() {
    let repo = tempfile::tempdir().unwrap();
    let outcome = run_setup(
        repo.path(),
        &SetupRequest {
            mode: SetupMode::Init,
            rewrite: false,
            reset_state: false,
        },
    )
    .unwrap();

    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
    assert!(!repo.path().join(".handbook/charter/CHARTER.md").exists());
    let charter = outcome
        .plan
        .artifacts
        .iter()
        .find(|row| row.artifact.instance_id == "project_authority")
        .expect("selected Charter action");
    assert_eq!(
        charter.action,
        handbook_compiler::SetupArtifactActionKind::AuthorRequired
    );
}

#[test]
fn doctor_adds_a_typed_missing_charter_observation_without_changing_readiness_rows() {
    let repo = tempfile::tempdir().unwrap();
    let report = doctor(repo.path()).unwrap();
    let charter = report.charter.expect("additive Charter row");

    assert_eq!(DOCTOR_REPORT_SCHEMA_VERSION, "1.2.0");
    assert_eq!(charter.instance_id, "project_authority");
    assert_eq!(
        charter.definition_closure_status,
        DoctorCharterDefinitionClosureStatus::Resolved
    );
    assert_eq!(charter.definition_closure.len(), 8);
    assert!(charter.definition_closure.iter().all(|record| {
        record.exact_ref.contains('@')
            && record.definition_fingerprint.starts_with("sha256:")
            && !record.package_path.is_empty()
    }));
    assert_eq!(charter.canonical_status, ArtifactInspectionStatus::Missing);
    assert_eq!(
        charter.canonical_reason,
        ArtifactInspectionReason::RequiredPathMissing
    );
    assert_eq!(
        charter.lifecycle_state,
        DoctorCharterLifecycleState::Unobserved
    );
    assert_eq!(charter.source_fingerprint, None);
    assert_eq!(charter.rendered_output_fingerprint, None);
    assert_eq!(
        charter.next_actions,
        vec![DoctorCharterNextAction::RunCharterAuthor]
    );
    let readiness_row = report
        .artifacts
        .iter()
        .find(|row| row.instance_id == "project_authority")
        .expect("retained readiness row");
    assert_eq!(readiness_row.inspection_status, charter.canonical_status);
    assert_eq!(readiness_row.inspection_reason, charter.canonical_reason);
}

#[test]
fn doctor_projects_valid_charter_bytes_from_one_retained_engine_observation() {
    let repo = tempfile::tempdir().unwrap();
    let selected = repo.path().join(".handbook/project/charter.yaml");
    std::fs::create_dir_all(selected.parent().unwrap()).unwrap();
    std::fs::write(&selected, CHARTER_YAML).unwrap();

    let report = doctor(repo.path()).unwrap();
    let charter = report.charter.expect("valid Charter row");

    assert_eq!(charter.canonical_path, ".handbook/project/charter.yaml");
    assert_eq!(
        charter.canonical_status,
        ArtifactInspectionStatus::StructurallyValid
    );
    assert_eq!(
        charter.canonical_reason,
        ArtifactInspectionReason::PresentAndStructurallyValid
    );
    assert!(charter
        .source_fingerprint
        .as_deref()
        .unwrap()
        .starts_with("sha256:"));
    assert!(charter
        .rendered_output_fingerprint
        .as_deref()
        .unwrap()
        .starts_with("sha256:"));
    assert_eq!(
        charter.rendered_media_type.as_deref(),
        Some("text/markdown")
    );
    assert_eq!(
        charter.next_actions,
        vec![DoctorCharterNextAction::ObserveLifecycle]
    );
}
